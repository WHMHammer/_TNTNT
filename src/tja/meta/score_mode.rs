#[derive(Debug, Clone, Copy)]
pub enum ScoreMode {
    Mode0, // AC 1 to AC 7
    Mode1, // AC 8 to AC 14
    Mode2, // AC 0
}

impl ScoreMode {
    pub fn from_str(slice: &str) -> Option<Self> {
        match slice {
            "0" => Some(Self::Mode0),
            "1" => Some(Self::Mode1),
            "2" => Some(Self::Mode2),
            _ => None,
        }
    }
}

// TODO: add score calculation calculations (using functional programming)

impl Default for ScoreMode {
    fn default() -> Self {
        Self::Mode1
    }
}
