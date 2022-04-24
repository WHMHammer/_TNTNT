pub mod load_tja;
pub use load_tja::*;

pub fn load_text_from_path<P>(
    path: P,
    encoding: Option<&'static encoding_rs::Encoding>,
) -> Option<String>
where
    P: AsRef<std::path::Path>,
{
    if let Ok(bytes) = std::fs::read(path) {
        if let Some(encoding) = encoding {
            let (cow, _, err) = encoding.decode(&bytes);
            if err {
                None
            } else {
                Some(cow.to_string())
            }
        } else {
            if let Ok(slice) = std::str::from_utf8(&bytes) {
                Some(slice.to_string())
            } else {
                let (cow, _, err) = encoding_rs::SHIFT_JIS.decode(&bytes);
                if err {
                    None
                } else {
                    Some(cow.to_string())
                }
            }
        }
    } else {
        None
    }
}
