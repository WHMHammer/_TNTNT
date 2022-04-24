use super::branch;

#[derive(Debug)]
pub enum Type {
    Don,             // 1
    Ka,              // 2
    DON,             // 3
    KA,              // 4
    Drumroll,        // 5
    DRUMROLL,        // 6
    Balloon,         // 7
    End,             // 8
    BALLOON,         // 9
    MEASURE(u8, u8), // numerator, denominator
    BPMCHANGE(f64),
    DELAY(f64),
    SCROLL(f64),
    GOGOSTART,
    GOGOEND,
    BARLINEOFF,
    BARLINEON,
    BRANCH(
        branch::Type,
        Vec<Event>,
        Vec<Event>,
        Vec<Event>
    ), // branch type (with thresholds), #N branch, #E branch, #M branch
    SECTION,
    LYRIC(String),
    LEVELHOLD,
    NEXTSONG(String, String, String, String, u32, u32), // title, subtitle, genre, audio filename, scoreinit, scorediff
}

#[derive(Debug)]
pub struct Event {
    pub offset: f64,
    pub event: Type,
}
