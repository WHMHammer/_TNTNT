use rodio::Source;

pub struct Audio {
    channels: u16,
    sample_rate: u32,
    samples: Vec<f32>,
}

impl Audio {
    pub fn load<P>(path: P) -> Result<&'static Self, ()>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(path).or(Err(()))?;
        let decoder = rodio::decoder::Decoder::new(file).or(Err(()))?;
        Ok(Box::leak(Box::new(Self {
            channels: decoder.channels(),
            sample_rate: decoder.sample_rate(),
            samples: decoder.convert_samples().collect(),
        })))
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f64(
            (self.samples.len() / self.channels as usize) as f64 / self.sample_rate as f64,
        )
    }

    pub fn source(&'static self) -> AudioSource {
        AudioSource {
            audio: self,
            index: 0,
        }
    }

    pub fn play(
        &'static self,
        stream_handle: &rodio::OutputStreamHandle,
    ) -> Result<(), rodio::PlayError> {
        stream_handle.play_raw(self.source())
    }
}

pub struct AudioSource {
    audio: &'static Audio,
    index: usize,
}

impl Iterator for AudioSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.audio.samples.get(self.index);
        if let Some(&sample) = sample {
            self.index += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for AudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.audio.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.audio.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(self.audio.duration())
    }
}
