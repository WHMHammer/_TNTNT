use super::CenterPositionConfigurations;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NotesConfigurations {
    don: Option<CenterPositionConfigurations>,
    ka: Option<CenterPositionConfigurations>,
    big_don: Option<CenterPositionConfigurations>,
    big_ka: Option<CenterPositionConfigurations>,
    drumroll_head: Option<CenterPositionConfigurations>,
    drumroll_body: Option<CenterPositionConfigurations>,
    drumroll_tail: Option<CenterPositionConfigurations>,
    big_drumroll_head: Option<CenterPositionConfigurations>,
    big_drumroll_body: Option<CenterPositionConfigurations>,
    big_drumroll_tail: Option<CenterPositionConfigurations>,
    balloon_head: Option<CenterPositionConfigurations>,
    balloon_tail: Option<CenterPositionConfigurations>,
    big_balloon_head: Option<CenterPositionConfigurations>,
    big_balloon_tail: Option<CenterPositionConfigurations>,
    mine: Option<CenterPositionConfigurations>,
    purple: Option<CenterPositionConfigurations>,
}

impl NotesConfigurations {
    pub fn load<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        if let Ok(file) = std::fs::File::open(path) {
            serde_json::from_reader(std::io::BufReader::new(file)).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}
