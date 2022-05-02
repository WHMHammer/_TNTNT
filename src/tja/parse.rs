use super::*;
use crate::i18n;

#[derive(PartialEq)]
pub enum State {
    Comment,
    MetaKey,
    MetaValue,
    CommandKey,
    CommandValue,
    Measure,
}

#[derive(Clone)]
pub struct Context {
    pub measure: (f64, f64), // numerator, denominator (#MEASURE numerator,denominator)
    pub bpm: f64,
    pub flag_barline: bool,
    pub measure_notes_count: u8,
    pub offset: f64,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            measure: (4.0, 4.0),
            bpm: 120.0,
            flag_barline: true,
            measure_notes_count: 0,
            offset: 0.0,
        }
    }
}

fn second_per_note(context: &Context) -> f64 {
    240.0 / context.bpm * context.measure.0 / context.measure.1 / context.measure_notes_count as f64
}

pub fn move_events<const FLAG_EXPLICIT_MEASURE: bool>(
    measure: &mut std::collections::VecDeque<course::event::EventType>,
    events: &mut Vec<course::Event>,
    context: &mut Context,
) {
    let mut offset = second_per_note(context);
    while let Some(event_type) = measure.pop_front() {
        use course::event::*;
        match event_type {
            Empty => {
                context.offset += offset;
            }
            Don | Ka | DON | KA | Drumroll | DRUMROLL | Balloon | End | BALLOON => {
                events.push(course::Event {
                    offset: context.offset,
                    event_type,
                });
                context.offset += offset;
            }
            MEASURE(numerator, denominator) => {
                context.measure = (numerator as f64, denominator as f64);
                offset = second_per_note(context);
            }
            BPMCHANGE(bpm) => {
                context.bpm = bpm;
                offset = second_per_note(context);
            }
            DELAY(delay) => {
                context.offset += delay;
            }
            BARLINEOFF => {
                context.flag_barline = false;
            }
            BARLINEON => {
                context.flag_barline = true;
            }
            _ => {
                events.push(course::Event {
                    offset: context.offset,
                    event_type,
                });
            }
        }
    }
    if FLAG_EXPLICIT_MEASURE {
        // am explicit measure with an ending comma
        if context.measure_notes_count == 0 {
            // an empty measure
            context.offset += 60.0 / context.bpm * context.measure.0 as f64;
        }
        if context.flag_barline {
            events.push(course::Event {
                offset: context.offset,
                event_type: course::event::BARLINE,
            });
        }
    } else if context.measure_notes_count != 0 {
        // an implicit measure without an ending comma
        if context.flag_barline {
            events.push(course::Event {
                offset: context.offset,
                event_type: course::event::BARLINE,
            });
        }
    }
    // otherwise not a measure, but just some commands; do nothing
    context.measure_notes_count = 0;
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
            let mut chart = Self::default(); // the chart to be returned
            chart.meta.genre = genre.map(String::clone); // some tja files don't record their genres, but they are grouped in different folders
            let mut char_indices = text.char_indices();
            let mut i = 0;
            let mut previous_character = ' ';
            let mut index_low = 0;
            let mut index_high = 0;
            let mut key = "";
            let mut course = Course::default(); // the current course working on; moved to the chart at the #END
            let mut events = &mut course.p0; // the current event buffer working on; switched at the #START [p1|p2]
            let mut course_events_ptr: *mut Vec<course::Event> = std::ptr::null_mut(); // events will point to the event buffer when #BRANCHSTART; this ptr records the course events to be written back at #BRANCHEND
            let mut measure = std::collections::VecDeque::new(); // the temporary buffer containing the events in the current measure; its contents will be moved to the events at the end of the measures ("," in tja)
            let mut flag_eof = false; // flag: end of file
            let mut state = State::MetaKey; // the "state" of the current line; one line records either a meta datum (e.g. TITLE:xxx) or a command (e.g. #MEASURE a,b) or some notes
            let mut context = Context::default(); // bpm, etc.
            let mut chart_context = Context::default(); // the initial context of the chart; cloned back to context at the #END
            let mut branchstart_context = Context::default(); // the context at #BRANCHSTART; cloned back to context when start parsing another branch
            loop {
                let character = if let Some((index, character)) = char_indices.next() {
                    i = index;
                    character
                } else {
                    // the file may not have an ending line break, in which case we simulate one
                    flag_eof = true;
                    i += 1;
                    '\n'
                };
                if character == '\n' {
                    let mut value = "";
                    if state != State::Comment {
                        if previous_character == '\r' {
                            index_high = i - 1;
                        } else {
                            index_high = i;
                        }
                    }
                    if key.is_empty() {
                        // always store commands without parameters in key
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
                                chart_context.bpm = bpm;
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
                            // TODO: fix the issue that the initial barline is inserted regardless of whether there is a #BARLINEOFF before the first note
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
                            _ => {
                                events = &mut course.p0;
                                events.push(course::Event {
                                    offset: 0.0,
                                    event_type: course::event::BARLINE,
                                });
                            }
                        },
                        "#END" => {
                            // TODO: handle STYLE:Double correctly
                            move_events::<false>(&mut measure, events, &mut context);
                            {
                                println!("{:?}", course.meta.course);
                                use std::io::Write;
                                let mut path = Vec::new();
                                write!(&mut path, "{:?}.out", course.meta.course).unwrap();
                                let mut file =
                                    std::fs::File::create(String::from_utf8(path).unwrap())
                                        .unwrap();
                                write!(&mut file, "{:#?}", course).unwrap();
                            }
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
                            events = &mut course.p0;
                            course_events_ptr = std::ptr::null_mut();
                            context = chart_context.clone();
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
                                move_events::<false>(&mut measure, events, &mut context);
                                unsafe {
                                    // cannot rewrite with safe code because course_events is (indirectly) owned by events, so we can't have &mut for both
                                    if let Some(course_events) = course_events_ptr.as_mut() {
                                        // handles the case where this #BRANCHSTART directly follows the previous one
                                        course_events.push(course::Event {
                                            offset: context.offset,
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
                                        // handles the case where this #BRANCHSTART does not directly follow another one
                                        events.push(course::Event {
                                            offset: context.offset,
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
                                branchstart_context = context.clone();
                            }
                        }
                        "#N" | "#E" | "#M" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                context = branchstart_context.clone();
                                move_events::<false>(&mut measure, events, &mut context);
                                if let course::event::BRANCH(branches) =
                                    &mut course_events.last_mut().unwrap().event_type
                                {
                                    events = match key {
                                        "#N" => &mut branches.n,
                                        "#E" => &mut branches.e,
                                        "#M" => &mut branches.m,
                                        _ => unreachable!(),
                                    };
                                } else {
                                    unreachable!();
                                }
                            }
                        },
                        "#BRANCHEND" => unsafe {
                            if let Some(course_events) = course_events_ptr.as_mut() {
                                move_events::<false>(&mut measure, events, &mut context);
                                events = course_events;
                                course_events_ptr = std::ptr::null_mut();
                            }
                        },
                        "#SECTION" => measure.push_back(course::event::SECTION), // TODO: investigate into its relation with #BRANCHSTART
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
                        _ => {}
                    }
                    /*if !key.is_empty() {
                        println!("{}\t{}", key, value);
                    }*/
                    index_low = i + 1;
                    state = State::MetaKey;
                    key = "";
                } else if state != State::Comment {
                    match character {
                        '/' => {
                            // change of line
                            if previous_character == '/' {
                                index_high = i - 1;
                                state = State::Comment;
                            }
                        }
                        ':' => {
                            if state == State::MetaKey {
                                key = &text[index_low..i];
                                index_low = i + 1;
                                state = State::MetaValue;
                            }
                        }
                        '#' => {
                            // if a line starts with "#," it records a command
                            index_low = i;
                            state = State::CommandKey;
                        }
                        ' ' => {
                            if previous_character == '\n' {
                                // ignore indentation
                                index_low = i + 1;
                                continue;
                            }
                            if state == State::CommandKey {
                                key = &text[index_low..i];
                                index_low = i + 1;
                                state = State::CommandValue;
                            }
                        }
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            // if a line starts with a number, it records some notes
                            if previous_character == '\n' {
                                state = State::Measure;
                            }
                            if state == State::Measure {
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
                            if state == State::Measure || previous_character == '\n' {
                                move_events::<true>(&mut measure, events, &mut context);
                            }
                        }
                        _ => {}
                    }
                }
                if flag_eof {
                    {
                        use std::io::Write;
                        let mut file = std::fs::File::create("Chart.out").unwrap();
                        write!(&mut file, "{:?}", chart).unwrap();
                    }
                    return Some(chart);
                }
                previous_character = character;
            }
        }
        None
    }
}
