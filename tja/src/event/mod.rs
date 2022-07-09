pub mod branch;
pub mod next_song;
use branch::Branches;
use next_song::NextSong;

pub enum EventType {
    Empty,         // 0
    Don,           // 1
    Ka,            // 2
    BigDon,        // 3
    BigKa,         // 4
    Drumroll,      // 5
    BigDrumroll,   // 6
    Balloon,       // 7
    End,           // 8
    BigBalloon,    // 9
    DualPlayerDon, // A
    DualPlayerKa,  // B
    Bomb,          // C
    ADLIB,         // F
    Purple,        // G
    Barline,
    Measure(u8, u8),
    BpmChange(f64),
    Delay(f64),
    Scroll(f64),
    GogoStart,
    GogoEnd,
    BarlineOff,
    BarlineOn,
    Branch(Branches),
    Section,
    Lyric(String),
    LevelHold,
    NextSong(NextSong),
}

impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventType::*;
        match self {
            Empty => write!(f, "0"),
            Don => write!(f, "1"),
            Ka => write!(f, "2"),
            BigDon => write!(f, "3"),
            BigKa => write!(f, "4"),
            Drumroll => write!(f, "5"),
            BigDrumroll => write!(f, "6"),
            Balloon => write!(f, "7"),
            End => write!(f, "8"),
            BigBalloon => write!(f, "9"),
            DualPlayerDon => write!(f, "A"),
            DualPlayerKa => write!(f, "B"),
            Bomb => write!(f, "C"),
            ADLIB => write!(f, "F"),
            Purple => write!(f, "G"),
            Barline => write!(f, ","),
            Measure(numerator, denominator) => write!(f, "#MEASURE {},{}", numerator, denominator),
            BpmChange(bpm) => write!(f, "#BPMCHANGE {}", bpm),
            Delay(delay) => write!(f, "#DELAY {}", delay),
            Scroll(scroll) => write!(f, "#SCROLL {}", scroll),
            GogoStart => write!(f, "#GOGOSTART"),
            GogoEnd => write!(f, "#GOGOEND"),
            BarlineOff => write!(f, "#BARLINEOFF"),
            BarlineOn => write!(f, "#BARLINEON"),
            Branch(branches) => write!(f, "{:?}", branches),
            Section => write!(f, "#SECTION"),
            Lyric(lyric) => write!(f, "#LYRIC {}", lyric),
            LevelHold => write!(f, "#LEVELHOLD"),
            NextSong(next_song) => write!(f, "{:?}", next_song),
        }
    }
}

pub struct Event {
    pub offset: f64, // delay before the event takes place
    pub event_type: EventType,
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventType::*;
        match self.event_type {
            Branch(_) => write!(f, "{:?}", self.event_type),
            NextSong(_) => write!(f, "\n{:?}", self.event_type),
            _ => write!(f, "{:?}\t@ {:.3}s", self.event_type, self.offset),
        }
    }
}
