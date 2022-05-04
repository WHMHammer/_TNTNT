pub mod sounds;
use sounds::Sounds;

pub struct Resources {
    pub sounds: Sounds,
}

impl Resources {
    pub fn load_from_directory<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        Self {
            sounds: Sounds::load_from_directory(path.join("Sounds")),
        }
    }
}
