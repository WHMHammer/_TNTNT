#[allow(non_snake_case)]
pub mod en_US;
#[allow(non_snake_case)]
pub mod zh_CN;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    zh_CN,
}
pub use Locale::*;

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct I18nString {
    default: String,
    en_US: Option<String>,
    zh_CN: Option<String>,
}

impl I18nString {
    pub fn set_default(&mut self, slice: &str) {
        self.default = slice.to_string();
    }

    pub fn set(&mut self, slice: &str, locale: Locale) {
        match locale {
            en_US => self.en_US = Some(slice.to_string()),
            zh_CN => self.zh_CN = Some(slice.to_string()),
        }
    }

    pub fn get(&self, locale: Locale) -> &String {
        match locale {
            en_US => &self.en_US,
            zh_CN => &self.zh_CN,
        }
        .as_ref()
        .unwrap_or(&self.default)
    }
}
