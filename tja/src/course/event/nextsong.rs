pub struct Nextsong {
    pub title: String,
    pub subtitle: String,
    pub genre: String,
    pub wave: String,
    pub scoreinit: u32,
    pub scorediff: u32,
}

impl Nextsong {
    pub fn from_str(slice: &str) -> Option<Self> {
        let mut values = slice.split(',');
        if let Some(title) = values.next() {
            if let Some(subtitle) = values.next() {
                if let Some(genre) = values.next() {
                    if let Some(wave) = values.next() {
                        if let Some(scoreinit) = values.next() {
                            if let Ok(scoreinit) = scoreinit.parse() {
                                if let Some(scorediff) = values.next() {
                                    if let Ok(scorediff) = scorediff.parse() {
                                        return Some(Self {
                                            title: title.to_string(),
                                            subtitle: subtitle.to_string(),
                                            genre: genre.to_string(),
                                            wave: wave.to_string(),
                                            scoreinit,
                                            scorediff,
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

impl std::fmt::Debug for Nextsong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#NEXTSONG {},{},{},{},{},{}",
            self.title, self.subtitle, self.genre, self.wave, self.scoreinit, self.scorediff
        )
    }
}
