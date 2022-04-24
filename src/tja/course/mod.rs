pub mod event;
pub mod meta;
pub mod branch;
pub use event::Event;

#[derive(Debug, Default)]
pub struct Course {
    pub meta: meta::Meta,
    pub p1: Vec<Event>,
    pub p2: Vec<Event>,
}
