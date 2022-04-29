#[derive(Default)]
pub struct Sounds {
    don: Option<std::io::Cursor<Vec<u8>>>,
    ka: Option<std::io::Cursor<Vec<u8>>>,
    balloon: Option<std::io::Cursor<Vec<u8>>>,
}

impl Sounds {
    pub fn load_from_directory<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut sounds = Self::default();
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            let path = entry.path();
                            if let Some(file_stem) = path.file_stem() {
                                if file_stem == "don" {
                                    sounds.don =
                                        Some(std::io::Cursor::new(std::fs::read(path).unwrap()))
                                } else if file_stem == "ka" {
                                    sounds.ka =
                                        Some(std::io::Cursor::new(std::fs::read(path).unwrap()))
                                } else if file_stem == "balloon" {
                                    sounds.balloon =
                                        Some(std::io::Cursor::new(std::fs::read(path).unwrap()))
                                }
                            }
                        }
                    }
                }
            }
        }
        sounds
    }

    pub fn play_don(&self, stream_handle: &rodio::OutputStreamHandle) {
        if let Some(don) = &self.don {
            if let Ok(decoder) = rodio::Decoder::new(don.clone()) {
                use rodio::Source;
                let _ = stream_handle.play_raw(decoder.convert_samples());
            }
        }
    }

    pub fn play_ka(&self, stream_handle: &rodio::OutputStreamHandle) {
        if let Some(don) = &self.ka {
            if let Ok(decoder) = rodio::Decoder::new(don.clone()) {
                use rodio::Source;
                let _ = stream_handle.play_raw(decoder.convert_samples());
            }
        }
    }
}

pub fn play_audio_from_path<P>(stream_handle: &rodio::OutputStreamHandle, path: P)
where
    P: AsRef<std::path::Path>,
{
    if let Ok(file) = std::fs::File::open(path) {
        if let Ok(decoder) = rodio::Decoder::new(std::io::BufReader::new(file)) {
            use rodio::Source;
            let _ = stream_handle.play_raw(decoder.convert_samples());
        }
    }
}
