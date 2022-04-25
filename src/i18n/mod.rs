#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    zh_CN,
}

pub use Locale::*;

impl Default for Locale {
    fn default() -> Self {
        en_US
    }
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct I18nString {
    en_US: Option<String>,
    zh_CN: Option<String>,
}

impl I18nString {
    pub fn set(&mut self, slice: &str, locale: Locale) {
        match locale {
            en_US => self.en_US = Some(slice.to_string()),
            zh_CN => self.zh_CN = Some(slice.to_string()),
        }
    }

    pub fn get(&self, locales: &[Locale]) -> &str {
        // locales in decreasing order of preference
        for locale in locales {
            if let Some(string) = match locale {
                en_US => &self.en_US,
                zh_CN => &self.zh_CN,
            } {
                return string;
            }
        }
        match Locale::default() {
            en_US => &self.en_US,
            zh_CN => &self.zh_CN,
        }
        .as_deref()
        .unwrap_or("[text not found]")
    }

    pub fn is_none(&self, locale: Locale) -> bool {
        match locale {
            en_US => &self.en_US,
            zh_CN => &self.zh_CN,
        }
        .is_none()
    }
}
