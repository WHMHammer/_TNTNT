pub mod course;
pub mod meta;
use crate::i18n;
pub use course::Course;
pub use meta::Meta;
mod parse;

#[derive(Debug, Default)]
pub struct Chart {
    pub meta: Meta,
    pub easy_course: Option<Course>,
    pub normal_course: Option<Course>,
    pub hard_course: Option<Course>,
    pub oni_course: Option<Course>,
    pub edit_course: Option<Course>,
    pub dan_course: Option<Course>,
    pub tower_course: Option<Course>,
}

impl Chart {
    pub fn parse_from_path<P>(
        path: P,
        encoding: Option<&'static encoding_rs::Encoding>,
        conf: &crate::conf::Conf,
        genre: Option<&String>,
    ) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        if let Ok(bytes) = std::fs::read(path) {
            let mut text = None;
            if let Some(encoding) = encoding {
                let (cow, flag_error) = encoding.decode_with_bom_removal(&bytes);
                if !flag_error {
                    text = Some(cow);
                }
            }
            if text.is_none() {
                let (cow, flag_error) = encoding_rs::UTF_8.decode_with_bom_removal(&bytes);
                if !flag_error {
                    text = Some(cow);
                }
            }
            if text.is_none() {
                let (cow, _, flag_error) = encoding_rs::SHIFT_JIS.decode(&bytes);
                if !flag_error {
                    text = Some(cow);
                }
            }
            if text.is_none() {
                return None;
            }
            let text = text.unwrap();
            let mut chart = Self::default();
            chart.meta.genre = genre.map(String::clone);
            let mut char_indices = text.char_indices();
            let mut i = 0;
            let mut previous_character = ' ';
            let mut index_low = 0;
            let mut index_high = 0;
            let mut key = "";
            let mut course = Course::default();
            let mut events = &mut course.p1;
            let mut course_events_ptr: *mut Vec<course::Event> = std::ptr::null_mut();
            let mut measure = std::collections::VecDeque::new();
            let mut flag_eof = false;
            let mut state = parse::State::Meta;
            let mut context = parse::Context::default();
            let mut course_context = context;
            let mut branchstart_context = course_context;
            loop {
                let character = if let Some((index, character)) = char_indices.next() {
                    i = index;
                    character
                } else {
                    // the file may not have an ending line break
                    flag_eof = true;
                    i += 1;
                    '\n'
                };
                if character == '\n' {
                    let mut value = "";
                    if state != parse::State::Comment {
                        if previous_character == '\r' {
                            index_high = i - 1;
                        } else {
                            index_high = i;
                        }
                    }
                    if key.is_empty() {
                        key = text[index_low..index_high].trim();
                    } else {
                        value = text[index_low..index_high].trim();
                    }
                    match key {
                        "TITLE" => {
                            let locale = conf.locales[0];
                            if chart.meta.title.is_none(locale) {
                                chart.meta.title.set(value, locale);
                            }
                        }
                        "TITLEEN" => {
                            chart.meta.title.set(value, i18n::en_US);
                        }
                        "TITLECN" => {
                            chart.meta.title.set(value, i18n::zh_CN);
                        }
                        "SUBTITLE" => {
                            chart.meta.subtitle.set(
                                value,
                                *conf.locales.first().unwrap_or(&i18n::Locale::default()),
                            );
                        }
                        "SUBTITLEEN" => {
                            chart.meta.subtitle.set(value, i18n::en_US);
                        }
                        "SUBTITLECN" => {
                            chart.meta.subtitle.set(value, i18n::zh_CN);
                        }
                        "BPM" => {
                            if let Ok(bpm) = value.parse() {
                                context.bpm = bpm;
                                course_context.bpm = bpm;
                            }
                        }
                        "WAVE" => {
                            chart.meta.wave = Some(value.to_string());
                        }
                        "OFFSET" => {
                            if let Ok(value) = value.parse() {
                                chart.meta.offset = value;
                            }
                        }
                        "DEMOSTART" => {
                            if let Ok(value) = value.parse() {
                                chart.meta.demostart = value;
                            }
                        }
                        "GENRE" => {
                            chart.meta.genre = Some(value.to_string());
                        }
                        "SCOREMODE" => {
                            if let Some(scoremode) = meta::ScoreMode::from_str(value) {
                                chart.meta.scoremode = scoremode;
                            }
                        }
                        "LIFE" => {
                            if let Ok(value) = value.parse() {
                                chart.meta.life = value;
                            }
                        }
                        "BGMOVIE" => {
                            chart.meta.bgmovie = Some(value.to_string());
                        }
                        "COURSE" => {
                            if let Some(c) = course::meta::Course::from_str(value) {
                                course.meta.course = c;
                            }
                        }
                        "LEVEL" => {
                            if let Ok(level) = value.parse() {
                                course.meta.level = level;
                            }
                        }
                        "BALLOON" => {
                            for value in value.split(',') {
                                if let Ok(value) = value.parse() {
                                    course.meta.balloon.push(value);
                                }
                            }
                        }
                        "SCOREINIT" => {
                            if let Ok(scoreinit) = value.parse() {
                                course.meta.scoreinit = scoreinit;
                            }
                        }
                        "SCOREDIFF" => {
                            if let Ok(scorediff) = value.parse() {
                                course.meta.scorediff = scorediff;
                            }
                        }
                        "STYLE" => {
                            if let Some(style) = course::meta::Style::from_str(value) {
                                course.meta.style = style;
                            }
                        }
                        "EXAM1" => {
                            course.meta.exam1 = course::meta::Exam::from_str(value);
                        }
                        "EXAM2" => {
                            course.meta.exam2 = course::meta::Exam::from_str(value);
                        }
                        "EXAM3" => {
                            course.meta.exam3 = course::meta::Exam::from_str(value);
                        }
                        "#START" => match value {
                            // TODO: fix the issue where the initial barline is inserted regardless of whether there is a #BARLINEOFF before the first note
                            "p1" => {
                                events = &mut course.p1;
                                events.push(course::Event {
                                    offset: 0.0,
                                    event_type: course::event::BARLINE,
                                });
                            }
                            "p2" => {
                                events = &mut course.p2;
                                events.push(course::Event {
                                    offset: 0.0,
                                    event_type: course::event::BARLINE,
                                });
                            }
                            _ => {}
                        },
                        "#END" => {
                            parse::move_events(&mut measure, events, &mut course_context);
                            use course::meta::Course::*;
                            match course.meta.course {
                                Easy => {
                                    chart.easy_course = Some(std::mem::take(&mut course));
                                }
                                Normal => {
                                    chart.normal_course = Some(std::mem::take(&mut course));
                                }
                                Hard => {
                                    chart.hard_course = Some(std::mem::take(&mut course));
                                }
                                Oni => {
                                    chart.oni_course = Some(std::mem::take(&mut course));
                                }
                                Edit => {
                                    chart.edit_course = Some(std::mem::take(&mut course));
                                }
                                Dan => {
                                    chart.dan_course = Some(std::mem::take(&mut course));
                                }
                                Tower => {
                                    chart.tower_course = Some(std::mem::take(&mut course));
                                }
                            }
                            events = &mut course.p1;
                            course_events_ptr = std::ptr::null_mut();
                            course_context = context;
                        }
                        "#MEASURE" => {
                            let mut values = value.split('/');
                            if let Some(numerator) = values.next() {
                                if let Ok(numerator) = numerator.parse() {
                                    if let Some(denominator) = values.next() {
                                        if let Ok(denominator) = denominator.parse() {
                                            measure.push_back(course::event::MEASURE(
                                                numerator,
                                                denominator,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        "#BPMCHANGE" => {
                            if let Ok(bpm) = value.parse() {
                                measure.push_back(course::event::BPMCHANGE(bpm));
                            }
                        }
                        "#DELAY" => {
                            if let Ok(delay) = value.parse() {
                                measure.push_back(course::event::DELAY(delay))
                            }
                        }
                        "#SCROLL" => {
                            if let Ok(scroll) = value.parse() {
                                measure.push_back(course::event::SCROLL(scroll))
                            }
                        }
                        "#GOGOSTART" => measure.push_back(course::event::GOGOSTART),
                        "#GOGOEND" => measure.push_back(course::event::GOGOEND),
                        "#BARLINEOFF" => measure.push_back(course::event::BARLINEOFF),
                        "#BARLINEON" => measure.push_back(course::event::BARLINEON),
                        "#BRANCHSTART" => {
                            if let Some(branches) = course::event::Branches::from_str(value) {
                                parse::move_events(&mut measure, events, &mut context);
                                unsafe {
                                    if let Some(course_events) = course_events_ptr.as_mut() {
                                        course_events.push(course::Event {
                                            offset: 0.0,
                                            event_type: course::event::BRANCH(branches),
                                        });
                                        if let course::event::BRANCH(branches) =
                                            &mut course_events.last_mut().unwrap().event_type
                                        {
                                            events = &mut branches.n;
                                        } else {
                                            unreachable!()
                                        }
                                    } else {
                                        events.push(course::Event {
                                            offset: 0.0,
                                            event_type: course::event::BRANCH(branches),
                                        });
                                        course_events_ptr = &mut *events;
                                        if let course::event::BRANCH(branches) =
                                            &mut course_events_ptr
                                                .as_mut()
                                                .unwrap()
                                                .last_mut()
                                                .unwrap()
                                                .event_type
                                        {
                                            events = &mut branches.n;
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                }
                                branchstart_context = course_context;
                            }
                        }
                        "#N" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                course_context = branchstart_context;
                                parse::move_events(&mut measure, events, &mut context);
                                if let course::event::BRANCH(branches) =
                                    &mut course_events.last_mut().unwrap().event_type
                                {
                                    events = &mut branches.n;
                                } else {
                                    unreachable!();
                                }
                            }
                        },
                        "#E" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                course_context = branchstart_context;
                                parse::move_events(&mut measure, events, &mut context);
                                if let course::event::BRANCH(branches) =
                                    &mut course_events.last_mut().unwrap().event_type
                                {
                                    events = &mut branches.e;
                                } else {
                                    unreachable!();
                                }
                            }
                        },
                        "#M" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                course_context = branchstart_context;
                                parse::move_events(&mut measure, events, &mut context);
                                if let course::event::BRANCH(branches) =
                                    &mut course_events.last_mut().unwrap().event_type
                                {
                                    events = &mut branches.m;
                                } else {
                                    unreachable!();
                                }
                            }
                        },
                        "#BRANCHEND" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                parse::move_events(&mut measure, events, &mut context);
                                events = course_events;
                                course_events_ptr = std::ptr::null_mut();
                            }
                        },
                        "#SECTION" => measure.push_back(course::event::SECTION),
                        "#LYRIC" => {
                            if !value.is_empty() {
                                measure.push_back(course::event::LYRIC(value.to_string()));
                            }
                        }
                        "#LEVELHOLD" => measure.push_back(course::event::LEVELHOLD),
                        "#NEXTSONG" => {
                            if let Some(nextsong) = course::event::Nextsong::from_str(value) {
                                measure.push_back(course::event::NEXTSONG(nextsong))
                            }
                        }
                        _ => {
                            if !key.is_empty() {
                                println!("{}", key);
                            }
                        }
                    }
                    index_low = i + 1;
                    state = parse::State::Meta;
                    key = "";
                } else if state != parse::State::Comment {
                    match character {
                        '/' => {
                            if previous_character == '/' {
                                index_high = i - 1;
                                state = parse::State::Comment;
                            }
                        }
                        ':' => {
                            key = &text[index_low..i];
                            index_low = i + 1;
                        }
                        '#' => {
                            index_low = i;
                            state = parse::State::CommandKey;
                        }
                        ' ' => {
                            if state == parse::State::CommandKey {
                                key = &text[index_low..i];
                                index_low = i + 1;
                                state = parse::State::CommandValue;
                            }
                        }
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            if previous_character == '\n' {
                                state = parse::State::Measure;
                            }
                            if state == parse::State::Measure {
                                measure.push_back(match character {
                                    '0' => course::event::Empty,
                                    '1' => course::event::Don,
                                    '2' => course::event::Ka,
                                    '3' => course::event::DON,
                                    '4' => course::event::KA,
                                    '5' => course::event::Drumroll,
                                    '6' => course::event::DRUMROLL,
                                    '7' => course::event::Balloon,
                                    '8' => course::event::End,
                                    '9' => course::event::BALLOON,
                                    _ => unreachable!(),
                                });
                                context.measure_notes_count += 1;
                            }
                        }
                        ',' => {
                            if state == parse::State::Measure {
                                parse::move_events(&mut measure, events, &mut context);
                            }
                        }
                        _ => {}
                    }
                }
                if flag_eof {
                    return Some(chart);
                }
                previous_character = character;
            }
        }
        None
    }
}

impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TJAChart {{")
            .and(writeln!(f, "    TITLE:{}", self.meta.title.get(&[])))
            .and(writeln!(f, "    SUBTITLE:{}", self.meta.subtitle.get(&[])))
            .and(writeln!(
                f,
                "    WAVE:{}",
                self.meta.wave.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(f, "    OFFSET:{}", self.meta.offset))
            .and(writeln!(f, "    DEMOSTART:{}", self.meta.demostart))
            .and(writeln!(
                f,
                "    GENRE:{}",
                self.meta.genre.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(f, "    SCOREMODE:{:?}", self.meta.scoremode))
            .and(writeln!(f, "    LIFE:{}", self.meta.life))
            .and(writeln!(
                f,
                "    BGMOVIE:{}",
                self.meta.bgmovie.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(
                f,
                "    Easy\tNormal\tHard\tOni\t\tEdit\tDan\t\tTower"
            ))
            .and(writeln!(
                f,
                "    {}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.easy_course.is_some(),
                self.normal_course.is_some(),
                self.hard_course.is_some(),
                self.oni_course.is_some(),
                self.edit_course.is_some(),
                self.dan_course.is_some(),
                self.tower_course.is_some(),
            ))
            .and(writeln!(f, "}}"))
    }
}
