pub mod event;
pub mod meta;
pub use event::Event;
pub use meta::Meta;

#[derive(Debug)]
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
