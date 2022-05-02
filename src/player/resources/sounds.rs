#[derive(Clone, Copy)]
pub enum Sound {
    Don,
    Ka,
    Balloon,
}
pub use Sound::*;

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
                                if file_stem == "dong" {
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

    pub fn play(&self, stream_handle: &rodio::OutputStreamHandle, sound: Sound) {
        if let Some(sound) = match sound {
            Don => &self.don,
            Ka => &self.ka,
            Balloon => &self.balloon,
        } {
            if let Ok(decoder) = rodio::Decoder::new(sound.clone()) {
                use rodio::Source;
                let _ = stream_handle.play_raw(decoder.convert_samples());
            }
        }
    }
}
