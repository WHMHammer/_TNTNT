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
            "Easy" | "easy" | "0" => Some(Self::Easy),
            "Normal" | "normal" | "1" => Some(Self::Normal),
            "Hard" | "hard" | "2" => Some(Self::Hard),
            "Oni" | "oni" | "4" => Some(Self::Oni),
            "Edit" | "edit" | "5" => Some(Self::Edit),
            "Dan" | "dan" | "6" => Some(Self::Dan),
            "Tower" | "tower" | "7" => Some(Self::Tower),
            _ => None,
        }
    }
}

impl Default for Course {
    fn default() -> Self {
        Self::Oni
    }
}
