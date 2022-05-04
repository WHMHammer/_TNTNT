// TODO: rewrite Audio to improve performance
use rodio::decoder::Decoder;
use rodio::Source;
use std::fs::read;
use std::io::Cursor;

#[derive(Clone)]
pub struct Audio(Cursor<Vec<u8>>);

impl Audio {
    pub fn load_from_path<P>(path: P) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        if let Ok(content) = read(path) {
            Some(Self(Cursor::new(content)))
        } else {
            None
        }
    }

    pub fn decoder(&self) -> Option<Decoder<Cursor<Vec<u8>>>> {
        Decoder::new(self.0.clone()).ok()
    }

    pub fn play(&self, stream_handle: &rodio::OutputStreamHandle) {
        if let Some(decoder) = self.decoder() {
            let _ = stream_handle.play_raw(decoder.convert_samples());
        }
    }
}
