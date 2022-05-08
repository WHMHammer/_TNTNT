use super::course;
use super::meta::scoremode::Scoremode;
use crate::i18n;
use course::{
    event::{branch::Branches, nextsong::Nextsong, Event, EventType},
    meta::{course_name::CourseName, exam::Exam, style::Style},
    Course,
};

#[derive(PartialEq)]
enum State {
    Comment,
    MetaKey,
    MetaValue,
    CommandKey,
    CommandValue,
    Measure,
}

#[derive(Clone, PartialEq)]
enum Branch {
    None,
    N,
    E,
    M,
}

#[derive(Clone, Copy)]
enum Player {
    P0,
    P1,
    P2,
}

#[derive(Clone)]
struct Context {
    time_signature: (f64, f64), // numerator, denominator (#MEASURE numerator,denominator)
    bpm: f64,
    measure_notes_count: u8,
    offset: f64,
    course: CourseName,
    branch: Branch,
    style: Style,
    player: Player,
}

impl Context {
    fn seconds_per_note(&self) -> f64 {
        240.0 / self.bpm * self.time_signature.0
            / self.time_signature.1
            / self.measure_notes_count as f64
    }

    fn move_events<const FLAG_EXPLICIT_MEASURE: bool>(
        &mut self,
        measure: &mut std::collections::VecDeque<EventType>,
        events: &mut Vec<Event>,
    ) {
        use EventType::*;
        let mut offset = self.seconds_per_note();
        while let Some(event_type) = measure.pop_front() {
            match event_type {
                Empty => {
                    self.offset += offset;
                }
                Don | Ka | DON | KA | Drumroll | DRUMROLL | Balloon | End | BALLOON => {
                    events.push(Event {
                        offset: self.offset,
                        event_type,
                    });
                    self.offset += offset;
                }
                MEASURE(numerator, denominator) => {
                    self.time_signature = (numerator as f64, denominator as f64);
                    offset = self.seconds_per_note();
                }
                BPMCHANGE(bpm) => {
                    self.bpm = bpm;
                    offset = self.seconds_per_note();
                }
                DELAY(delay) => {
                    self.offset += delay;
                }
                NEXTSONG(_) => {
                    self.offset = 0.0;
                    events.push(Event {
                        offset: 0.0,
                        event_type,
                    });
                }
                _ => {
                    events.push(Event {
                        offset: self.offset,
                        event_type,
                    });
                }
            }
        }
        if FLAG_EXPLICIT_MEASURE {
            // an explicit measure with an ending comma
            if self.measure_notes_count == 0 {
                // an empty measure
                self.offset += 60.0 / self.bpm * self.time_signature.0 as f64;
            }
            events.push(Event {
                offset: self.offset,
                event_type: MEASUREEND,
            });
        } else if self.measure_notes_count != 0 {
            // an implicit measure without an ending comma
            events.push(Event {
                offset: self.offset,
                event_type: MEASUREEND,
            });
        }
        // otherwise not a measure, but just some commands; no need to increase the offset
        self.measure_notes_count = 0;
    }

    fn get_events_mut<'course>(&self, course: &'course mut Course) -> &'course mut Vec<Event> {
        if self.branch == Branch::None {
            match self.player {
                Player::P0 => &mut course.p0,
                Player::P1 => &mut course.p1,
                Player::P2 => &mut course.p2,
            }
        } else if let EventType::BRANCH(branches) = &mut match self.player {
            Player::P0 => &mut course.p0,
            Player::P1 => &mut course.p1,
            Player::P2 => &mut course.p2,
        }
        .last_mut()
        .unwrap()
        .event_type
        {
            match self.branch {
                Branch::N => &mut branches.n,
                Branch::E => &mut branches.e,
                Branch::M => &mut branches.m,
                Branch::None => unreachable!(),
            }
        } else {
            panic!()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            time_signature: (4.0, 4.0),
            bpm: 120.0,
            measure_notes_count: 0,
            offset: 0.0,
            course: CourseName::default(),
            branch: Branch::None,
            style: Style::default(),
            player: Player::P0,
        }
    }
}

impl super::Chart {
    pub fn parse_from_path<P>(
        path: P,
        encoding: Option<&'static encoding_rs::Encoding>,
        conf: &crate::application::conf::Conf,
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
            chart.meta.genre = genre.map(String::clone); // some tja files don't record their genres, but rather determined by their folders
            let mut char_indices = text.char_indices();
            let mut i = 0;
            let mut previous_character = ' ';
            let mut index_low = 0;
            let mut index_high = 0;
            let mut key = "";
            let mut flag_eof = false; // flag: end of file
            let mut state = State::MetaKey; // the "state" of the current line; one line records either a meta datum (e.g. TITLE:xxx) or a command (e.g. #MEASURE a,b) or some notes
            let mut context = Context::default();
            let mut chart_context = Context::default(); // the initial context of the chart; cloned back to `context` at the #END
            let mut branchstart_context = Context::default(); // the context at #BRANCHSTART; cloned back to `context` when shifting to another branch
            let mut measure = std::collections::VecDeque::new(); // the temporary buffer containing the events in the current measure; its contents will be moved to `chart` at the end of the measures ("," in tja)
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
                        // always store commands in key
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
                            if let Some(scoremode) = Scoremode::from_str(value) {
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
                            if let Some(c) = CourseName::from_str(value) {
                                chart.get_course_mut(c).meta.course = c;
                                context.course = c;
                            }
                        }
                        "LEVEL" => {
                            if let Ok(level) = value.parse() {
                                chart.get_course_mut(context.course).meta.level = level;
                            }
                        }
                        "BALLOON" => {
                            for value in value.split(',') {
                                if let Ok(value) = value.parse() {
                                    if context.style == Style::Single {
                                        chart
                                            .get_course_mut(context.course)
                                            .meta
                                            .balloon
                                            .push(value);
                                    } else {
                                        chart
                                            .get_course_mut(context.course)
                                            .meta
                                            .balloon_double
                                            .push(value);
                                    }
                                }
                            }
                        }
                        "SCOREINIT" => {
                            if let Ok(scoreinit) = value.parse() {
                                if context.style == Style::Single {
                                    chart.get_course_mut(context.course).meta.scoreinit = scoreinit;
                                } else {
                                    chart.get_course_mut(context.course).meta.scoreinit_double =
                                        scoreinit;
                                }
                            }
                        }
                        "SCOREDIFF" => {
                            if let Ok(scorediff) = value.parse() {
                                if context.style == Style::Single {
                                    chart.get_course_mut(context.course).meta.scorediff = scorediff;
                                } else {
                                    chart.get_course_mut(context.course).meta.scorediff_double =
                                        scorediff;
                                }
                            }
                        }
                        "STYLE" => {
                            if let Some(s) = Style::from_str(value) {
                                context.style = s;
                            }
                        }
                        "EXAM1" => {
                            chart.get_course_mut(context.course).meta.exam1 = Exam::from_str(value);
                        }
                        "EXAM2" => {
                            chart.get_course_mut(context.course).meta.exam2 = Exam::from_str(value);
                        }
                        "EXAM3" => {
                            chart.get_course_mut(context.course).meta.exam3 = Exam::from_str(value);
                        }
                        "#START" => {
                            // TODO: insert the initial barline when necessary
                            match value {
                                "P1" => context.player = Player::P1,
                                "P2" => context.player = Player::P2,
                                _ => context.player = Player::P0,
                            }
                        }
                        "#END" => {
                            context.move_events::<false>(
                                &mut measure,
                                context.get_events_mut(chart.get_course_mut(context.course)),
                            );
                            {
                                use std::io::Write;
                                println!("{:?}", context.course);
                                let mut path = Vec::new();
                                write!(&mut path, "{:?}.out", context.course).unwrap();
                                let mut file =
                                    std::fs::File::create(String::from_utf8(path).unwrap())
                                        .unwrap();
                                write!(
                                    &mut file,
                                    "{:#?}",
                                    chart.get_course(context.course).unwrap()
                                )
                                .unwrap();
                            }
                            let course = context.course;
                            context = chart_context.clone(); // the STYLE info is lost, but it doesn't matter
                            context.course = course; // the COURSE info has to be kept for STYLE:Double
                        }
                        "#MEASURE" => {
                            let mut values = value.split('/');
                            if let Some(numerator) = values.next() {
                                if let Ok(numerator) = numerator.parse() {
                                    if let Some(denominator) = values.next() {
                                        if let Ok(denominator) = denominator.parse() {
                                            measure.push_back(EventType::MEASURE(
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
                                measure.push_back(EventType::BPMCHANGE(bpm));
                            }
                        }
                        "#DELAY" => {
                            if let Ok(delay) = value.parse() {
                                measure.push_back(EventType::DELAY(delay))
                            }
                        }
                        "#SCROLL" => {
                            if let Ok(scroll) = value.parse() {
                                measure.push_back(EventType::SCROLL(scroll))
                            }
                        }
                        "#GOGOSTART" => measure.push_back(EventType::GOGOSTART),
                        "#GOGOEND" => measure.push_back(EventType::GOGOEND),
                        "#BARLINEOFF" => measure.push_back(EventType::BARLINEOFF),
                        "#BARLINEON" => measure.push_back(EventType::BARLINEON),
                        "#BRANCHSTART" => {
                            if let Some(branches) = Branches::from_str(value) {
                                let course = chart.get_course_mut(context.course);
                                context.move_events::<false>(
                                    &mut measure,
                                    context.get_events_mut(course),
                                );
                                match context.player {
                                    Player::P0 => &mut course.p0,
                                    Player::P1 => &mut course.p1,
                                    Player::P2 => &mut course.p2,
                                }
                                .push(Event {
                                    offset: context.offset,
                                    event_type: EventType::BRANCH(branches),
                                });
                                context.branch = Branch::N;
                                branchstart_context = context.clone();
                            }
                        }
                        "#N" | "#E" | "#M" => {
                            if context.branch != Branch::None {
                                context.move_events::<false>(
                                    &mut measure,
                                    context.get_events_mut(chart.get_course_mut(context.course)),
                                );
                                context = branchstart_context.clone();
                                context.branch = match key {
                                    "#N" => Branch::N,
                                    "#E" => Branch::E,
                                    "#M" => Branch::M,
                                    _ => unreachable!(),
                                };
                            }
                        }
                        "#BRANCHEND" => {
                            if context.branch != Branch::None {
                                context.move_events::<false>(
                                    &mut measure,
                                    context.get_events_mut(chart.get_course_mut(context.course)),
                                );
                                context.branch = Branch::None;
                            }
                        }
                        "#SECTION" => measure.push_back(EventType::SECTION), // TODO: investigate into its relation with #BRANCHSTART
                        "#LYRIC" => {
                            if !value.is_empty() {
                                measure.push_back(EventType::LYRIC(value.to_string()));
                            }
                        }
                        "#LEVELHOLD" => measure.push_back(EventType::LEVELHOLD),
                        "#NEXTSONG" => {
                            if let Some(nextsong) = Nextsong::from_str(value) {
                                measure.push_back(EventType::NEXTSONG(nextsong))
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
                            if previous_character == '\n' {
                                state = State::Measure;
                            }
                            if state == State::Measure {
                                use EventType::*;
                                measure.push_back(match character {
                                    '0' => Empty,
                                    '1' => Don,
                                    '2' => Ka,
                                    '3' => DON,
                                    '4' => KA,
                                    '5' => Drumroll,
                                    '6' => DRUMROLL,
                                    '7' => Balloon,
                                    '8' => End,
                                    '9' => BALLOON,
                                    _ => unreachable!(),
                                });
                                context.measure_notes_count += 1;
                            }
                        }
                        ',' => {
                            if state == State::Measure || previous_character == '\n' {
                                context.move_events::<true>(
                                    &mut measure,
                                    context.get_events_mut(chart.get_course_mut(context.course)),
                                );
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
