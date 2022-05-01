pub mod branch;
pub mod nextsong;
pub use branch::Branches;
pub use nextsong::Nextsong;

#[derive(Debug)]
pub enum EventType {
    Empty,    // 0
    Don,      // 1
    Ka,       // 2
    DON,      // 3
    KA,       // 4
    Drumroll, // 5
    DRUMROLL, // 6
    Balloon,  // 7
    End,      // 8
    BALLOON,  // 9
    MEASURE(u8, u8),
    BPMCHANGE(f64),
    DELAY(f64),
    SCROLL(f64),
    GOGOSTART,
    GOGOEND,
    BARLINEOFF,
    BARLINEON,
    BARLINE,
    BRANCH(Branches),
    SECTION,
    LYRIC(String),
    LEVELHOLD,
    NEXTSONG(Nextsong),
}
pub use EventType::*;

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Don
            | Self::Ka
            | Self::DON
            | Self::KA
            | Self::Drumroll
            | Self::DRUMROLL
            | Self::Balloon
            | Self::End
            | Self::BALLOON => {
                write!(f, "{:?}", self)
            }
            Self::MEASURE(numerator, denominator) => {
                write!(f, "#MEASURE {},{}", numerator, denominator)
            }
            Self::BPMCHANGE(bpm) => {
                write!(f, "#BPMCHANGE {}", bpm)
            }
            Self::DELAY(delay) => {
                write!(f, "#DELAY {}", delay)
            }
            Self::SCROLL(scroll) => {
                write!(f, "#SCROLL {}", scroll)
            }
            Self::BARLINE => {
                write!(f, ",")
            }
            Self::BRANCH(branches) => match branches.thresholds {
                branch::Thresholds::r(e, m) => {
                    writeln!(f, "#BRANCHSTART r,{},{}", e, m)
                }
                branch::Thresholds::p(e, m) => {
                    writeln!(f, "#BRANCHSTART p,{},{}", e, m)
                }
            }
            .and(writeln!(f, "#N"))
            .and({
                let mut iter = branches.n.iter();
                loop {
                    if let Some(event) = iter.next() {
                        let r = writeln!(f, "{:#?}", event);
                        if r.is_err() {
                            break r;
                        }
                    } else {
                        break Ok(());
                    }
                }
            })
            .and(writeln!(f, "#E"))
            .and({
                let mut iter = branches.e.iter();
                loop {
                    if let Some(event) = iter.next() {
                        let r = writeln!(f, "{:#?}", event);
                        if r.is_err() {
                            break r;
                        }
                    } else {
                        break Ok(());
                    }
                }
            })
            .and(writeln!(f, "#M"))
            .and({
                let mut iter = branches.m.iter();
                loop {
                    if let Some(event) = iter.next() {
                        let r = writeln!(f, "{:#?}", event);
                        if r.is_err() {
                            break r;
                        }
                    } else {
                        break Ok(());
                    }
                }
            })
            .and(write!(f, "#BRANCHEND")),
            Self::LYRIC(lyric) => {
                write!(f, "#LYRIC {}", lyric)
            }
            Self::NEXTSONG(nextsong) => {
                write!(
                    f,
                    "#NEXTSONG {},{},{},{},{},{}",
                    nextsong.title,
                    nextsong.subtitle,
                    nextsong.genre,
                    nextsong.wave,
                    nextsong.scoreinit,
                    nextsong.scorediff
                )
            }
            _ => {
                write!(f, "#{:?}", self)
            }
        }
    }
}

pub struct Event {
    pub offset: f64, // delay before the event takes place
    pub event_type: EventType,
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}\t\t@ {:.3}", self.event_type, self.offset)
        } else {
            write!(f, "{:?} @ {:.3}", self.event_type, self.offset)
        }
    }
}
