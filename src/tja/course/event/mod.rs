pub mod branch;
pub mod nextsong;
pub use branch::Branches;
pub use nextsong::Nextsong;

#[derive(Debug)]
pub enum EventType {
    Don,      // 1
    Ka,       // 2
    DON,      // 3
    KA,       // 4
    Drumroll, // 5
    DRUMROLL, // 6
    Balloon,  // 7
    End,      // 8
    BALLOON,  // 9
    DELAY,
    SCROLL(f64),
    GOGOSTART,
    GOGOEND,
    BARLINE,
    BRANCH(Branches),
    SECTION,
    LYRIC(String),
    LEVELHOLD,
    NEXTSONG(Nextsong),
    DEBUG(String),
}
pub use EventType::*;

#[derive(Debug)]
pub struct Event {
    pub offset: f64, // delay before the event takes place
    pub event_type: EventType,
}
