#[derive(Debug)]
pub enum Style {
    Single,
    Double,
}

impl Style {
    pub fn from_str(slice: &str) -> Option<Self> {
        match slice {
            "Single" | "1" => Some(Self::Single),
            "Double" | "Couple" | "2" => Some(Self::Double),
            _ => None,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Single
    }
}
