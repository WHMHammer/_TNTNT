#[derive(Debug)]
pub enum SubtitleAppearance {
    Default,
    Hide,
    Show,
}

#[derive(Debug)]
pub enum BranchingType {
    r,
    p,
}

#[derive(Debug)]
pub struct Branches {
    normal: Option<Box<EventNode>>,
    advanced: Option<Box<EventNode>>,
    master: Option<Box<EventNode>>,
    branching_type: BranchingType,
    threshold_advanced: i32,
    threshold_master: i32,
}

#[derive(Debug)]
pub enum Event {
    Blank,
    Don,
    Ka,
    DON,
    KA,
    Drumroll,
    DRUMROLL,
    Balloon,
    End,
    BPMCHANGE(f64),
    BRANCH(Branches),
}

#[derive(Debug)]
pub struct EventNode {
    event: Event,
    next: Option<Box<EventNode>>,
}

#[derive(Debug)]
pub struct Chart {
    title: String,
    subtitle: String,
    subtitle_appearance: SubtitleAppearance,
    bpm: f64,
    wave: String,
    offset: f64,
    easy_course: Option<Box<EventNode>>,
    normal_course: Option<Box<EventNode>>,
    hard_course: Option<Box<EventNode>>,
    oni_course: Option<Box<EventNode>>,
    edit_course: Option<Box<EventNode>>,
}

impl Default for Chart {
    fn default() -> Chart {
        Chart {
            title: String::new(),
            subtitle: String::new(),
            subtitle_appearance: SubtitleAppearance::Default,
            bpm: 120.0,
            wave: String::new(),
            offset: 0.0,
            easy_course: None,
            normal_course: None,
            hard_course: None,
            oni_course: None,
            edit_course: None,
        }
    }
}

impl Chart {
    pub fn from_string(tja_string: &String) -> Result<Chart, ()> {
        let mut chart = Chart::default();
        let mut index_low = 0;
        let mut key: Option<&str> = None;
        let mut flag_first_slash = false;
        let mut flag_comment = false;
        let mut flag_command = false;
        let mut flag_course_started = false;
        let mut note_count = 0;
        for (i, character) in tja_string.char_indices() {
            if character == '\n' {
                if !flag_comment {
                    if let (Some(k), Some(v)) = (key, tja_string.get(index_low..i)) {
                        let v = v.trim();
                        match k.trim() {
                            "TITLE" => {
                                chart.title = v.into();
                            }
                            "SUBTITLE" => match v.get(0..2) {
                                Some("--") => {
                                    chart.subtitle_appearance = SubtitleAppearance::Hide;
                                    if let Some(v) = v.get(2..) {
                                        chart.subtitle = v.into();
                                    }
                                }
                                Some("++") => {
                                    chart.subtitle_appearance = SubtitleAppearance::Show;
                                    if let Some(v) = v.get(2..) {
                                        chart.subtitle = v.into();
                                    }
                                }
                                _ => {
                                    chart.subtitle = v.into();
                                }
                            },
                            "BPM" => {
                                if let Ok(value) = v.parse() {
                                    chart.bpm = value;
                                }
                            }
                            "WAVE" => {
                                chart.wave = v.into();
                            }
                            "OFFSET" => {
                                if let Ok(value) = v.parse() {
                                    chart.offset = value;
                                }
                            }
                            "COURSE" => match v {
                                "Easy" | "0" => {}
                                "Normal" | "1" => {}
                                "Hard" | "2" => {}
                                "Oni" | "3" => {}
                                "Edit" | "Ura" | "4" => {}
                                _ => {}
                            },
                            "LEVEL" => {}
                            "BALLOON" => {}
                            _ => {}
                        }
                    } else {
                        if let Some(line) = tja_string.get(index_low..i) {
                            match line.trim() {
                                "#START" => {
                                    index_low = i + 1;
                                    flag_course_started = true;
                                    note_count = 0;
                                }
                                "#END" => {
                                    flag_course_started = false;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if !flag_course_started {
                    index_low = i + 1;
                }
                key = None;
                flag_first_slash = false;
                flag_comment = false;
                flag_command = false;
            } else if !flag_comment {
                match character {
                    '/' => {
                        if flag_first_slash {
                            flag_comment = true;
                        } else {
                            flag_first_slash = true;
                        }
                    }
                    ':' => {
                        key = tja_string.get(index_low..i);
                        index_low = i + 1;
                    }
                    '#' => {
                        flag_command = true;
                    }
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'A' | 'B' | 'F' => {
                        if !flag_command {
                            note_count += 1;
                        }
                    }
                    ',' => {
                        if flag_course_started {
                            if let Some(measure) = tja_string.get(index_low..i) {
                                let measure = measure.trim();
                                println!("{}", measure);
                                println!("{} notes\n", note_count);
                            }
                            index_low = i + 1;
                            note_count = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(chart)
    }

    pub fn from_path<P>(path: P) -> Result<Chart, ()>
    where
        P: AsRef<std::path::Path>,
    {
        let tja_bytes = std::fs::read(path).unwrap();
        if let Ok(slice) = std::str::from_utf8(&tja_bytes) {
            Chart::from_string(&slice.to_string())
        } else {
            let (cow, _encoding_used, had_errors) = encoding_rs::SHIFT_JIS.decode(&tja_bytes);
            if had_errors {
                return Err(());
            }
            Chart::from_string(&cow.to_string())
        }
    }
}
