use crate::i18n;
use crate::tja;

#[derive(Debug)]
pub struct Conf {
    pub locales: Vec<i18n::Locale>,
    pub scoremode: tja::meta::ScoreMode,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            locales: vec![i18n::Locale::default()],
            scoremode: tja::meta::ScoreMode::default(),
        }
    }
}
