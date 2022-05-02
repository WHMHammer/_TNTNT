pub mod event;
pub mod meta;
pub use event::Event;
pub use meta::Meta;

pub struct Course {
    pub meta: Meta,
    pub events: Vec<Event>,
}

impl Default for Course {
    fn default() -> Self {
        Self {
            meta: Meta::default(),
            events: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.meta)?;
        for event in &self.events {
            write!(f, "\n{:?}", event)?;
        }
        Ok(())
    }
}
