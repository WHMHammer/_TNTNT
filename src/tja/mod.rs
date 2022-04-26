pub mod course;
pub mod meta;
use crate::i18n;
pub use course::Course;
pub use meta::Meta;

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
    pub fn from_path<P>(
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
            let mut index_low = 0;
            let mut index_high = 0;
            let mut key = "";
            let mut course = Course::default();
            let mut events = &mut course.p1;
            let mut measure: (u8, u8) = (4, 4); // TODO: define the default measure somewhere else
            let mut bpm = 120.0; // TODO: define the default BPM somewhere else
            let mut previous_character = ' ';
            let mut flag_eof = false;
            let mut flag_comment = false;
            let mut flag_command = false;
            let mut flag_barline = true;
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
                    if !flag_comment {
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
                            if let Ok(value) = value.parse() {
                                bpm = value;
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
                            "p1" => {
                                events = &mut course.p1;
                            }
                            "p2" => {
                                events = &mut course.p2;
                            }
                            _ => {}
                        },
                        "#END" => {
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
                            measure = (4, 4); // TODO: define the default measure somewhere else
                            bpm = 120.0; // TODO: define the default BPM somewhere else
                            flag_barline = true;
                        }
                        "#MEASURE" => {
                            let mut values = value.split('/');
                            if let Some(numerator) = values.next() {
                                if let Ok(numerator) = numerator.parse() {
                                    if let Some(denominator) = values.next() {
                                        if let Ok(denominator) = denominator.parse() {
                                            measure = (numerator, denominator);
                                        }
                                    }
                                }
                            }
                        }
                        "#BPMCHANGE" => {
                            if let Ok(b) = value.parse() {
                                bpm = b;
                            }
                        }
                        "#DELAY" => {
                            if let Ok(offset) = value.parse() {
                                events.push(course::Event {
                                    offset,
                                    event_type: course::event::EventType::DELAY,
                                })
                            }
                        }
                        "#SCROLL" => {
                            if let Ok(scroll) = value.parse() {
                                events.push(course::Event {
                                    offset: 0.0,
                                    event_type: course::event::EventType::SCROLL(scroll),
                                })
                            }
                        }
                        "#GOGOSTART" => events.push(course::Event {
                            offset: 0.0,
                            event_type: course::event::EventType::GOGOSTART,
                        }),
                        "#GOGOEND" => events.push(course::Event {
                            offset: 0.0,
                            event_type: course::event::EventType::GOGOEND,
                        }),
                        "#BARLINEOFF" => {
                            flag_barline = false;
                        }
                        "BARLINEON" => {
                            flag_barline = true;
                        }
                        //"#BRANCHSTART" => {

                        //}
                        //"#N" => {

                        //}
                        //"#E" => {

                        //}
                        //"#M" => {

                        //}
                        //"#BRANCHEND" => {

                        //}
                        "#SECTION" => events.push(course::Event {
                            offset: 0.0,
                            event_type: course::event::EventType::SECTION,
                        }),
                        "#LYRIC" => {
                            if !value.is_empty() {
                                events.push(course::Event {
                                    offset: 0.0,
                                    event_type: course::event::EventType::LYRIC(value.to_string()),
                                });
                            }
                        }
                        "#LEVELHOLD" => events.push(course::Event {
                            offset: 0.0,
                            event_type: course::event::EventType::LEVELHOLD,
                        }),
                        "#NEXTSONG" => {
                            let mut values = value.split(',');
                            if let Some(title) = values.next() {
                                if let Some(subtitle) = values.next() {
                                    if let Some(genre) = values.next() {
                                        if let Some(wave) = values.next() {
                                            if let Some(scoreinit) = values.next() {
                                                if let Ok(scoreinit) = scoreinit.parse() {
                                                    if let Some(scorediff) = values.next() {
                                                        if let Ok(scorediff) = scorediff.parse() {
                                                            events.push(course::Event {
                                                                offset: 0.0,
                                                                event_type:
                                                                    course::event::EventType::NEXTSONG(
                                                                        title.to_string(),
                                                                        subtitle.to_string(),
                                                                        genre.to_string(),
                                                                        wave.to_string(),
                                                                        scoreinit,
                                                                        scorediff,
                                                                    ),
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            if !key.is_empty() {
                                //println!("{:?}: {:?}", key, value);
                            }
                        }
                    }
                    index_low = i + 1;
                    flag_comment = false;
                    key = "";
                    flag_command = false;
                } else if !flag_comment {
                    match character {
                        '/' => {
                            if previous_character == '/' {
                                index_high = i - 1;
                                flag_comment = true;
                            }
                        }
                        ':' => {
                            key = &text[index_low..i];
                            index_low = i + 1;
                        }
                        '#' => {
                            index_low = i;
                            flag_command = true;
                        }
                        ' ' => {
                            if flag_command {
                                key = &text[index_low..i];
                                index_low = i + 1;
                                flag_command = false;
                            }
                        }
                        // TODO: add notes and comma
                        // don't forget the barline event at the end of the measure
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
