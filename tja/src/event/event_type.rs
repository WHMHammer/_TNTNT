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
    BarLine,
    Measure(u8, u8),
    BpmChange(f64),
    Delay(f64),
    Scroll(f64),
    GogoStart,
    GogoEnd,
    BarLineOff,
    BarLineOn,
    Branch(super::branch::Branches),
    Section,
    Lyric(String),
    LevelHold,
    NextSong(super::next_song::NextSong),
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
            BarLine => write!(f, ","),
            Measure(numerator, denominator) => write!(f, "#MEASURE {},{}", numerator, denominator),
            BpmChange(bpm) => write!(f, "#BPMCHANGE {}", bpm),
            Delay(delay) => write!(f, "#DELAY {}", delay),
            Scroll(scroll) => write!(f, "#SCROLL {}", scroll),
            GogoStart => write!(f, "#GOGOSTART"),
            GogoEnd => write!(f, "#GOGOEND"),
            BarLineOff => write!(f, "#BARLINEOFF"),
            BarLineOn => write!(f, "#BARLINEON"),
            Branch(branches) => write!(f, "{:?}", branches),
            Section => write!(f, "#SECTION"),
            Lyric(lyric) => write!(f, "#LYRIC {}", lyric),
            LevelHold => write!(f, "#LEVELHOLD"),
            NextSong(next_song) => write!(f, "{:?}", next_song),
        }
    }
}
