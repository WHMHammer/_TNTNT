#[derive(Debug)]
pub enum Course {
    Easy,   // かんたん
    Normal, // ふつう
    Hard,   // むずかしい
    Oni,    // おに（表）
    Edit,   // おに（裏）
    Dan,    // 段位道場
    Tower,  // 太鼓タワー
}

impl Course {
    pub fn from_str(slice: &str) -> Option<Self> {
        match slice {
            "Easy" | "0" => Some(Self::Easy),
            "Normal" | "1" => Some(Self::Normal),
            "Hard" | "2" => Some(Self::Hard),
            "Oni" | "Ura" | "Extreme" | "4" => Some(Self::Oni),
            "Edit" | "Extra" | "5" => Some(Self::Edit),
            "Dan" | "6" => Some(Self::Dan),
            "Tower" => Some(Self::Tower),
            _ => None,
        }
    }
}

impl Default for Course {
    fn default() -> Self {
        Self::Oni
    }
}
