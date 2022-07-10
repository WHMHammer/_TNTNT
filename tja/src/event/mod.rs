pub mod branch;
pub mod context;
pub mod event_type;
pub mod next_song;

pub struct Event {
    pub context: context::Context,
    pub event_type: event_type::EventType,
    pub time_offset: f64,     // seconds from starting time
    pub position_offset: f64, // unit lengths (the length of a measure under #MEASURE 4/4 and #SCROLL 1) from starting bar line
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use event_type::EventType::*;
        match self.event_type {
            Branch(_) => write!(f, "{:?}", self.event_type),
            Section | Lyric(_) => write!(f, "{:?}", self.event_type),
            NextSong(_) => write!(f, "\n{:?}", self.event_type),
            _ => write!(
                f,
                "{:?} @ {:07.3}s, {:07.3} unit lengths {:?}",
                self.event_type, self.time_offset, self.position_offset, self.context
            ),
        }
    }
}
