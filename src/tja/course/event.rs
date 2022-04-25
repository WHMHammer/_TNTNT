use super::branch;

#[derive(Debug)]
pub enum Type {
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
    BRANCH(branch::Type, f64, f64, Vec<Event>, Vec<Event>, Vec<Event>), // branch type, #E threshold, #M threshold, #N branch, #E branch, #M branch
    SECTION,
    LYRIC(String),
    LEVELHOLD,
    NEXTSONG(String, String, String, String, u32, u32), // title, subtitle, genre, wave, scoreinit, scorediff
}

#[derive(Debug)]
pub struct Event {
    pub offset: f64, // delay before the event takes place
    pub event: Type,
}
