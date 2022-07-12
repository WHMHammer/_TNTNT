use super::CenterPositionConfigurations;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct LaneConfigurations {
    pub lane: Option<CenterPositionConfigurations>,
    pub e_branch: Option<CenterPositionConfigurations>,
    pub m_branch: Option<CenterPositionConfigurations>,
    pub barline: Option<CenterPositionConfigurations>,
}

impl LaneConfigurations {
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
