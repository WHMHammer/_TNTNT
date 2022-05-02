#[derive(Clone, Copy)]
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

impl Default for ScoreMode {
    fn default() -> Self {
        Self::Mode1
    }
}

impl std::fmt::Debug for ScoreMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mode0 => write!(f, "0"),
            Self::Mode1 => write!(f, "1"),
            Self::Mode2 => write!(f, "2"),
        }
    }
}
