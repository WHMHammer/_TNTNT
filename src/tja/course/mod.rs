pub mod event;
pub mod meta;
pub use event::Event;
pub use meta::Meta;

pub struct Course {
    pub meta: Meta,
    pub p0: Vec<Event>,
    pub p1: Vec<Event>,
    pub p2: Vec<Event>,
}

impl Default for Course {
    fn default() -> Self {
        Self {
            meta: Meta::default(),
            p0: Vec::new(),
            p1: Vec::with_capacity(0),
            p2: Vec::with_capacity(0),
        }
    }
}

impl std::fmt::Debug for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.meta)?;
        if !self.p0.is_empty() {
            write!(f, "\n\np0:")?;
            for event in &self.p0 {
                write!(f, "\n{:?}", event)?;
            }
        }
        if !self.p1.is_empty() {
            write!(f, "\n\np1:")?;
            for event in &self.p1 {
                write!(f, "\n{:?}", event)?;
            }
        }
        if !self.p2.is_empty() {
            write!(f, "\n\np2:")?;
            for event in &self.p2 {
                write!(f, "\n{:?}", event)?;
            }
        }
        Ok(())
    }
}
