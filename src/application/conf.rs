use crate::i18n::Locale;
use crate::tja::meta::scoremode::Scoremode;

#[derive(Debug)]
pub struct Conf {
    pub locales: Vec<Locale>,
    pub scoremode: Scoremode,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            locales: vec![Locale::default()],
            scoremode: Scoremode::default(),
        }
    }
}
