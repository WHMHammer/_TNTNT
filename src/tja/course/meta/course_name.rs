#[derive(Clone, Copy, Debug)]
pub enum CourseName {
    Easy,
    Normal,
    Hard,
    Oni,
    Edit,
    Dan,
    Tower,
}

impl CourseName {
    pub fn from_str(slice: &str) -> Option<Self> {
        match slice {
            "Easy" | "easy" | "0" => Some(Self::Easy),
            "Normal" | "normal" | "1" => Some(Self::Normal),
            "Hard" | "hard" | "2" => Some(Self::Hard),
            "Oni" | "oni" | "3" => Some(Self::Oni),
            "Edit" | "edit" | "4" => Some(Self::Edit),
            "Tower" | "tower" | "5" => Some(Self::Tower),
            "Dan" | "dan" | "6" => Some(Self::Dan),
            _ => None,
        }
    }
}

impl Default for CourseName {
    fn default() -> Self {
        Self::Oni
    }
}
