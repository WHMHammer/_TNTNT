use hound::{Sample, SampleFormat::*, WavReader};
use lewton::inside_ogg::OggStreamReader;
use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};
use std::{collections::VecDeque, fs::File, path::Path, time::Duration};

struct Audio {
    samples: Vec<i16>,
    cursors: VecDeque<usize>,
}

impl AudioCallback for Audio {
    type Channel = i16;
    fn callback(&mut self, out_buffer: &mut [Self::Channel]) {
        for out in out_buffer.iter_mut() {
            *out = 0;
            for cursor in self.cursors.iter_mut() {
                *out += self.samples[*cursor];
                *cursor += 1;
            }
            while let Some(&cursor) = self.cursors.front() {
                if cursor == self.samples.len() {
                    self.cursors.pop_front();
                } else {
                    break;
                }
            }
        }
    }
}

pub struct Device {
    device: AudioDevice<Audio>,
    duration: f64,
}

impl Device {
    pub fn from_path<P>(path: P, audio_subsystem: &AudioSubsystem) -> Result<Self, ()>
    where
        P: AsRef<Path>,
    {
        if let Some(extension) = path.as_ref().extension() {
            if let Ok(file) = File::open(&path) {
                let mut samples = Vec::new();
                let (sample_rate, channels_count) = if extension == "wav" {
                    if let Ok(reader) = WavReader::new(file) {
                        let spec = reader.spec();
                        match spec.sample_format {
                            Float => {
                                for sample in reader.into_samples::<f32>() {
                                    if let Ok(sample) = sample {
                                        samples.push(sample.as_i16());
                                    } else {
                                        break;
                                    }
                                }
                            }
                            Int => {
                                if spec.bits_per_sample <= 16 {
                                    for sample in reader.into_samples::<i16>() {
                                        if let Ok(sample) = sample {
                                            samples.push(sample);
                                        } else {
                                            break;
                                        }
                                    }
                                } else {
                                    for sample in reader.into_samples::<i32>() {
                                        if let Ok(sample) = sample {
                                            samples.push(sample.as_i16());
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        (spec.sample_rate as i32, spec.channels as u8)
                    } else {
                        return Err(());
                    }
                } else if extension == "ogg" {
                    if let Ok(mut reader) = OggStreamReader::new(file) {
                        while let Ok(Some(mut packet)) = reader.read_dec_packet_itl() {
                            samples.append(&mut packet);
                        }
                        (
                            reader.ident_hdr.audio_sample_rate as i32,
                            reader.ident_hdr.audio_channels as u8,
                        )
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                };
                let samples_count = samples.len();
                let device = audio_subsystem
                    .open_playback(
                        None,
                        &AudioSpecDesired {
                            freq: Some(sample_rate),
                            channels: Some(channels_count),
                            samples: None,
                        },
                        move |_sepc| Audio {
                            samples,
                            cursors: VecDeque::new(),
                        },
                    )
                    .unwrap();
                device.resume();
                Ok(Self {
                    device,
                    duration: (samples_count / channels_count as usize) as f64 / sample_rate as f64,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.duration)
    }

    pub fn play(&mut self) {
        self.device.lock().cursors.push_back(0);
    }
}
