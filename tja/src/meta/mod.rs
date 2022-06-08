pub mod scoremode;
use i18n::{I18nString, Locale};
use scoremode::Scoremode;

pub struct Meta {
    pub title: I18nString,
    pub subtitle: I18nString,
    pub wave: Option<String>,
    pub offset: f64,
    pub demostart: f64,
    pub genre: Option<String>,
    pub scoremode: Scoremode,
    pub life: u8,
    pub bgmovie: Option<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            title: I18nString::default(),
            subtitle: I18nString::default(),
            wave: None,
            offset: 0.0,
            demostart: 0.0,
            genre: None,
            scoremode: Scoremode::default(),
            life: 0,
            bgmovie: None,
        }
    }
}

impl std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TITLE:{}", self.title.get(&[]))?;
        for &locale in Locale::variants() {
            if self.title.exists(locale) {
                writeln!(f, "TITLE{}:{}", locale.suffix(), self.title.get(&[locale]))?;
            }
        }
        writeln!(f, "SUBTITLE:{}", self.subtitle.get(&[]))?;
        for &locale in Locale::variants() {
            if self.title.exists(locale) {
                writeln!(
                    f,
                    "SUBTITLE{}:{}",
                    locale.suffix(),
                    self.subtitle.get(&[locale])
                )?;
            }
        }
        if let Some(wave) = &self.wave {
            writeln!(f, "WAVE:{}", wave)?;
        }
        if self.offset != 0.0 {
            writeln!(f, "OFFSET:{}", self.offset)?;
        }
        if self.demostart != 0.0 {
            writeln!(f, "DEMOSTART:{}", self.demostart)?;
        }
        if let Some(genre) = &self.genre {
            writeln!(f, "GENRE:{}", genre)?;
        }
        write!(f, "SCOREMODE:{:?}", self.scoremode)?;
        if self.life != 0 {
            write!(f, "\nLIFE:{}", self.life)?;
        }
        if let Some(bgmovie) = &self.bgmovie {
            write!(f, "\nBGMOVIE:{}", bgmovie)?;
        }
        Ok(())
    }
}
