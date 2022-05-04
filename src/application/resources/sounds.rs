use crate::application::audio::Audio;

#[derive(Clone, Copy)]
pub enum Sound {
    Don,
    Ka,
    Balloon,
}

#[derive(Default)]
pub struct Sounds {
    don: Option<Audio>,
    ka: Option<Audio>,
    balloon: Option<Audio>,
}

impl Sounds {
    pub fn load_from_directory<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        Self {
            don: Audio::load_from_path(path.join("Taiko/dong.ogg")),
            ka: Audio::load_from_path(path.join("Taiko/ka.ogg")),
            balloon: Audio::load_from_path(path.join("balloon.ogg")),
        }
    }

    pub fn play(&self, stream_handle: &rodio::OutputStreamHandle, sound: Sound) {
        use Sound::*;
        if let Some(audio) = match sound {
            Don => &self.don,
            Ka => &self.ka,
            Balloon => &self.balloon,
        } {
            audio.play(stream_handle);
        }
    }
}
