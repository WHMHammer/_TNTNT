pub mod score_mode;
pub use score_mode::ScoreMode;

use crate::i18n::I18nString;

#[derive(Debug)]
pub struct Meta {
    pub title: I18nString,
    pub subtitle: I18nString,
    pub bpm: f64,
    pub wave: Option<String>,
    pub offset: f64,
    pub demostart: f64,
    pub genre: Option<String>,
    pub scoremode: ScoreMode,
    pub life: u8,
    pub bgmovie: Option<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            title: I18nString::default(),
            subtitle: I18nString::default(),
            bpm: 120.0,
            wave: None,
            offset: 0.0,
            demostart: 0.0,
            genre: None,
            scoremode: ScoreMode::Mode1,
            life: 0,
            bgmovie: None,
        }
    }
}
