#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    zh_CN,
}
use Locale::*;

impl Locale {
    pub fn variants() -> &'static [Locale] {
        &[en_US, zh_CN]
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            en_US => "EN",
            zh_CN => "CN",
        }
    }
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct I18nString {
    default: String,
    en_US: Option<String>,
    zh_CN: Option<String>,
}

impl I18nString {
    fn _get(&self, locale: Locale) -> &Option<String> {
        match locale {
            en_US => &self.en_US,
            zh_CN => &self.zh_CN,
        }
    }

    fn _get_mut(&mut self, locale: Locale) -> &mut Option<String> {
        match locale {
            en_US => &mut self.en_US,
            zh_CN => &mut self.zh_CN,
        }
    }

    pub fn set(&mut self, slice: &str, locale: Option<Locale>) {
        if let Some(locale) = locale {
            *self._get_mut(locale) = Some(slice.to_string());
        } else {
            self.default = slice.to_string();
        }
    }

    pub fn exists(&self, locale: Locale) -> bool {
        self._get(locale).is_some()
    }

    pub fn get(&self, locales: &[Locale]) -> &str {
        // locales in decreasing order of preference
        for &locale in locales {
            if let Some(string) = self._get(locale) {
                return string;
            }
        }
        &self.default
    }
}
