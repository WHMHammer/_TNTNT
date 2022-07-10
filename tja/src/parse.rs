use crate::course::{
    meta::{difficulty::Difficulty, exam::Exam, style::Style},
    Course,
};
use crate::event::{
    branch::Branches, context::Context, event_type::EventType, next_song::NextSong, Event,
};
use crate::meta::scoremode::Scoremode;

#[derive(PartialEq)]
enum State {
    Comment,
    MetaValue,
    CommandKey,
    CommandValue,
    Notes,
}

#[derive(Clone, PartialEq)]
enum Branch {
    None,
    N,
    E,
    M,
}

#[derive(Clone, Copy, Debug)]
enum Player {
    P0,
    P1,
    P2,
}

#[derive(Clone)]
struct ParserContext {
    measure_notes_count: u8,
    time_offset: f64,
    position_offset: f64,
    time_signature: (f64, f64), // numerator, denominator (#MEASURE numerator,denominator)
    bpm: f64,
    event_context: Context,
    flag_bar_line: bool,
    course: Difficulty,
    branch: Branch,
    style: Style,
    player: Player,
}

impl ParserContext {
    fn seconds_per_note(&self) -> f64 {
        240.0 / self.bpm * self.time_signature.0
            / self.time_signature.1
            / self.measure_notes_count as f64
    }

    fn unit_lengths_per_note(&self) -> f64 {
        // one unit length is the length of a measure under #MEASURE 4/4 and #SCROLL 1
        self.time_signature.0 / self.time_signature.1 / self.measure_notes_count as f64
            * self.event_context.scroll
    }

    fn end_measure(&mut self, events: &mut Vec<Event>) {
        self.event_context.measure_index += 1;
        if self.flag_bar_line {
            events.push(Event {
                context: self.event_context.clone(),
                event_type: EventType::BarLine,
                time_offset: self.time_offset,
                position_offset: self.position_offset,
            });
        }
    }

    fn move_events<const FLAG_EXPLICIT_MEASURE: bool>(
        &mut self,
        measure: &mut std::collections::VecDeque<EventType>,
        events: &mut Vec<Event>,
    ) {
        use EventType::*;
        let mut seconds_per_note = self.seconds_per_note();
        let mut unit_lengths_per_note = self.unit_lengths_per_note();
        while let Some(event_type) = measure.pop_front() {
            match event_type {
                Empty => {
                    self.time_offset += seconds_per_note;
                    self.position_offset += unit_lengths_per_note;
                }
                Don | Ka | BigDon | BigKa | Drumroll | BigDrumroll | Balloon | End | BigBalloon
                | DualPlayerDon | DualPlayerKa | Bomb | ADLIB | Purple => {
                    events.push(Event {
                        context: self.event_context.clone(),
                        event_type,
                        time_offset: self.time_offset,
                        position_offset: self.position_offset,
                    });
                    self.time_offset += seconds_per_note;
                    self.position_offset += unit_lengths_per_note;
                }
                Measure(numerator, denominator) => {
                    self.time_signature = (numerator as f64, denominator as f64);
                    seconds_per_note = self.seconds_per_note();
                    unit_lengths_per_note = self.unit_lengths_per_note();
                }
                BpmChange(bpm) => {
                    self.bpm = bpm;
                    seconds_per_note = self.seconds_per_note();
                }
                Delay(delay) => {
                    self.time_offset += delay;
                    self.position_offset += delay / seconds_per_note * unit_lengths_per_note;
                }
                Scroll(scroll) => {
                    self.event_context.scroll = scroll;
                    unit_lengths_per_note = self.unit_lengths_per_note();
                }
                GogoStart => {
                    self.event_context.flag_gogo = true;
                }
                GogoEnd => {
                    self.event_context.flag_gogo = false;
                }
                BarLineOff => {
                    self.flag_bar_line = false;
                }
                BarLineOn => {
                    self.flag_bar_line = true;
                }
                NextSong(_) => {
                    if let Some(event) = events.last() {
                        if let EventType::BarLine = event.event_type {
                            if event.context.measure_index == 1 {
                                events.pop();
                            }
                        }
                    } else {
                        unreachable!();
                    }
                    self.time_offset = 0.0;
                    self.position_offset = 0.0;
                    self.event_context.measure_index = 1;
                    events.push(Event {
                        context: self.event_context.clone(),
                        event_type,
                        time_offset: 0.0,
                        position_offset: 0.0,
                    });
                    events.push(Event {
                        context: self.event_context.clone(),
                        event_type: EventType::BarLine,
                        time_offset: 0.0,
                        position_offset: 0.0,
                    });
                }
                _ => {
                    events.push(Event {
                        context: self.event_context.clone(),
                        event_type,
                        time_offset: self.time_offset,
                        position_offset: self.position_offset,
                    });
                }
            }
        }
        if FLAG_EXPLICIT_MEASURE {
            // an explicit measure with an ending comma
            if self.measure_notes_count == 0 {
                // an empty measure
                self.measure_notes_count = 1;
                self.time_offset += self.seconds_per_note();
                self.position_offset += self.unit_lengths_per_note();
            }
            self.end_measure(events);
        } else if self.measure_notes_count != 0 {
            // an implicit measure without an ending comma
            self.end_measure(events);
        } else {
            // otherwise not a measure, but just some commands; no need to increase the offsets
        }
        self.measure_notes_count = 0;
    }

    fn get_events_mut<'course>(&self, course: &'course mut Course) -> &'course mut Vec<Event> {
        if self.branch == Branch::None {
            match self.player {
                Player::P0 => &mut course.p0,
                Player::P1 => &mut course.p1,
                Player::P2 => &mut course.p2,
            }
        } else if let EventType::Branch(branches) = &mut match self.player {
            Player::P0 => &mut course.p0,
            Player::P1 => &mut course.p1,
            Player::P2 => &mut course.p2,
        }
        .last_mut()
        .unwrap() // TODO: consider the None case
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

impl Default for ParserContext {
    fn default() -> Self {
        Self {
            measure_notes_count: 0,
            time_offset: 0.0,
            position_offset: 0.0,
            time_signature: (4.0, 4.0),
            bpm: 120.0,
            event_context: Context::default(),
            flag_bar_line: true,
            course: Difficulty::default(),
            branch: Branch::None,
            style: Style::default(),
            player: Player::P0,
        }
    }
}

impl crate::Chart {
    pub fn load<P>(path: P, encoding: Option<&'static encoding_rs::Encoding>) -> Result<Self, ()>
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
                return Err(());
            }
            let text = text.unwrap(); // never panics because the None case is checkout above
            let mut chart = Self::default();
            let mut char_indices = text.char_indices();
            let mut i = 0;
            let mut previous_character = ' ';
            let mut index_low = 0;
            let mut index_high = 0;
            let mut key = "";
            let mut flag_eof = false; // flag: end of file
            let mut state = State::Notes; // the "state" of the current line; one line records either a meta datum (e.g. TITLE:xxx) or a command (e.g. #MEASURE a,b) or some notes
            let mut context = ParserContext::default();
            let mut chart_context = ParserContext::default(); // the initial context of the chart; cloned back to `context` at the #END
            let mut branch_start_context = ParserContext::default(); // the context at #BRANCHSTART; cloned back to `context` when shifting to another branch
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
                    use i18n::Locale::*;
                    match key {
                        "TITLE" => {
                            chart.meta.title.set(value, None);
                        }
                        "TITLEEN" => {
                            chart.meta.title.set(value, Some(en_US));
                        }
                        "TITLECN" => {
                            chart.meta.title.set(value, Some(zh_CN));
                        }
                        "SUBTITLE" => {
                            chart.meta.subtitle.set(value, None);
                        }
                        "SUBTITLEEN" => {
                            chart.meta.subtitle.set(value, Some(en_US));
                        }
                        "SUBTITLECN" => {
                            chart.meta.subtitle.set(value, Some(zh_CN));
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
                            if let Some(c) = Difficulty::from_str(value) {
                                chart.get_course_mut(c).meta.course = c;
                                context.course = c;
                            }
                        }
                        "LEVEL" => {
                            if let Ok(level) = value.parse() {
                                chart.get_course_mut(context.course).meta.level = level;
                            }
                        }
                        "BigBalloon" => {
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
                            if let Ok(score_init) = value.parse() {
                                if context.style == Style::Single {
                                    chart.get_course_mut(context.course).meta.score_init =
                                        score_init;
                                } else {
                                    chart.get_course_mut(context.course).meta.score_init_double =
                                        score_init;
                                }
                            }
                        }
                        "SCOREDIFF" => {
                            if let Ok(score_diff) = value.parse() {
                                if context.style == Style::Single {
                                    chart.get_course_mut(context.course).meta.score_diff =
                                        score_diff;
                                } else {
                                    chart.get_course_mut(context.course).meta.score_diff_double =
                                        score_diff;
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
                            match value {
                                "P1" => context.player = Player::P1,
                                "P2" => context.player = Player::P2,
                                _ => context.player = Player::P0,
                            }
                            measure.push_back(EventType::BarLine);
                        }
                        "#END" => {
                            context.move_events::<false>(
                                &mut measure,
                                context.get_events_mut(chart.get_course_mut(context.course)),
                            );
                            {
                                use std::io::Write;
                                println!("{:?} {:?}", context.course, context.player);
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
                                            measure.push_back(EventType::Measure(
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
                                measure.push_back(EventType::BpmChange(bpm));
                            }
                        }
                        "#DELAY" => {
                            if let Ok(delay) = value.parse() {
                                measure.push_back(EventType::Delay(delay))
                            }
                        }
                        "#SCROLL" => {
                            if let Ok(scroll) = value.parse() {
                                measure.push_back(EventType::Scroll(scroll))
                            }
                        }
                        "#GOGOSTART" => measure.push_back(EventType::GogoStart),
                        "#GOGOEND" => measure.push_back(EventType::GogoEnd),
                        "#BARLINEOFF" => measure.push_back(EventType::BarLineOff),
                        "#BARLINEON" => measure.push_back(EventType::BarLineOn),
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
                                    context: context.event_context.clone(),
                                    event_type: EventType::Branch(branches),
                                    time_offset: context.time_offset,
                                    position_offset: context.position_offset,
                                });
                                context.branch = Branch::N;
                                branch_start_context = context.clone();
                            }
                        }
                        "#N" | "#E" | "#M" => {
                            if context.branch != Branch::None {
                                context.move_events::<false>(
                                    &mut measure,
                                    context.get_events_mut(chart.get_course_mut(context.course)),
                                );
                                context = branch_start_context.clone();
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
                        "#SECTION" => measure.push_back(EventType::Section),
                        "#LYRIC" => {
                            if !value.is_empty() {
                                measure.push_back(EventType::Lyric(value.to_string()));
                            }
                        }
                        "#LEVELHOLD" => measure.push_back(EventType::LevelHold),
                        "#NEXTSONG" => {
                            if let Some(next_song) = NextSong::from_str(value) {
                                let course = chart.get_course_mut(context.course);
                                context.move_events::<false>(
                                    &mut measure,
                                    context.get_events_mut(course),
                                );
                                measure.push_back(EventType::NextSong(next_song));
                            }
                        }
                        _ => {
                            if state == State::Notes && key != "" {
                                for character in key.chars() {
                                    if character == ',' {
                                        context.move_events::<true>(
                                            &mut measure,
                                            context.get_events_mut(
                                                chart.get_course_mut(context.course),
                                            ),
                                        );
                                    } else {
                                        use EventType::*;
                                        measure.push_back(match character {
                                            '1' => Don,
                                            '2' => Ka,
                                            '3' => BigDon,
                                            '4' => BigKa,
                                            '5' => Drumroll,
                                            '6' => BigDrumroll,
                                            '7' => Balloon,
                                            '8' => End,
                                            '9' => BigBalloon,
                                            'A' => DualPlayerDon,
                                            'B' => DualPlayerKa,
                                            'C' => Bomb,
                                            'F' => ADLIB,
                                            'G' => Purple,
                                            _ => Empty,
                                        });
                                        context.measure_notes_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    index_low = i + 1;
                    state = State::Notes;
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
                            if state == State::Notes {
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
                        _ => {}
                    }
                }
                if flag_eof {
                    {
                        use std::io::Write;
                        let mut file = std::fs::File::create("Chart.out").unwrap();
                        write!(&mut file, "{:?}", chart).unwrap();
                    }
                    return Ok(chart);
                }
                previous_character = character;
            }
        }
        Err(())
    }
}
