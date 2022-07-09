pub struct NextSong {
    pub title: String,
    pub subtitle: String,
    pub genre: String,
    pub wave: String,
    pub score_init: u32,
    pub score_diff: u32,
}

impl NextSong {
    pub fn from_str(slice: &str) -> Option<Self> {
        let mut values = slice.split(',');
        if let Some(title) = values.next() {
            if let Some(subtitle) = values.next() {
                if let Some(genre) = values.next() {
                    if let Some(wave) = values.next() {
                        if let Some(score_init) = values.next() {
                            if let Ok(score_init) = score_init.parse() {
                                if let Some(score_diff) = values.next() {
                                    if let Ok(score_diff) = score_diff.parse() {
                                        return Some(Self {
                                            title: title.to_string(),
                                            subtitle: subtitle.to_string(),
                                            genre: genre.to_string(),
                                            wave: wave.to_string(),
                                            score_init,
                                            score_diff,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

impl std::fmt::Debug for NextSong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#NEXTSONG {},{},{},{},{},{}",
            self.title, self.subtitle, self.genre, self.wave, self.score_init, self.score_diff
        )
    }
}
