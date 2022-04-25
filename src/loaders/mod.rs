use crate::i18n;
use crate::tja;

pub fn load_tja_from_path<P>(
    path: P,
    encoding: Option<&'static encoding_rs::Encoding>,
    conf: &crate::conf::Conf,
    genre: Option<&String>,
) -> Option<tja::Chart>
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
        let mut chart = tja::Chart::default();
        chart.meta.genre = genre.map(String::clone);
        chart.meta.scoremode = conf.scoremode;
        let mut char_indices = text.char_indices();
        let mut i = 0;
        let mut index_low = 0;
        let mut index_high = 0;
        let mut key = "";
        let mut course = tja::course::Course::default();
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
                        if let Some(mode) = tja::meta::ScoreMode::from_str(value) {
                            chart.meta.scoremode = mode;
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
                        if let Some(c) = tja::course::meta::Course::from_str(value) {
                            course.meta.course = c;
                        }
                    }
                    "LEVEL" => {
                        if let Ok(value) = value.parse() {
                            course.meta.level = value;
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
                        if let Ok(value) = value.parse() {
                            course.meta.scoreinit = value;
                        }
                    }
                    "SCOREDIFF" => {
                        if let Ok(value) = value.parse() {
                            course.meta.scorediff = value;
                        }
                    }
                    "STYLE" => {
                        if let Some(style) = tja::course::meta::Style::from_str(value) {
                            course.meta.style = style;
                        }
                    }
                    "EXAM1" => {
                        course.meta.exam1 = tja::course::meta::Exam::from_str(value);
                    }
                    "EXAM2" => {
                        course.meta.exam2 = tja::course::meta::Exam::from_str(value);
                    }
                    "EXAM3" => {
                        course.meta.exam3 = tja::course::meta::Exam::from_str(value);
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
                        use tja::course::meta::Course::*;
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
                        if let Ok(value) = value.parse() {
                            bpm = value;
                        }
                    }
                    "#DELAY" => {
                        if let Ok(value) = value.parse() {
                            events.push(tja::course::Event {
                                offset: value,
                                event: tja::course::event::Type::DELAY,
                            })
                        }
                    }
                    "#SCROLL" => {
                        if let Ok(value) = value.parse() {
                            events.push(tja::course::Event {
                                offset: 0.0,
                                event: tja::course::event::Type::SCROLL(value),
                            })
                        }
                    }
                    "#GOGOSTART" => events.push(tja::course::Event {
                        offset: 0.0,
                        event: tja::course::event::Type::GOGOSTART,
                    }),
                    "#GOGOEND" => events.push(tja::course::Event {
                        offset: 0.0,
                        event: tja::course::event::Type::GOGOEND,
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
                    "#SECTION" => events.push(tja::course::Event {
                        offset: 0.0,
                        event: tja::course::event::Type::SECTION,
                    }),
                    "#LYRIC" => {
                        if !value.is_empty() {
                            events.push(tja::course::Event {
                                offset: 0.0,
                                event: tja::course::event::Type::LYRIC(value.to_string()),
                            });
                        }
                    }
                    "#LEVELHOLD" => events.push(tja::course::Event {
                        offset: 0.0,
                        event: tja::course::event::Type::LEVELHOLD,
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
                                                        events.push(tja::course::Event {
                                                            offset: 0.0,
                                                            event:
                                                                tja::course::event::Type::NEXTSONG(
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

pub fn get_all_tja_paths(root: &str) -> Vec<std::path::PathBuf> {
    let conf = crate::conf::Conf::default();
    let mut paths = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(std::path::PathBuf::from(root));
    while let Some(directory) = queue.pop_front() {
        if let Ok(entires) = directory.read_dir() {
            for entry in entires {
                if let Ok(entry) = entry {
                    if let Ok(filetype) = entry.file_type() {
                        let path = entry.path();
                        if filetype.is_dir() {
                            queue.push_back(path);
                        } else if filetype.is_file() {
                            if let Some(extension) = path.extension() {
                                if extension == "tja" {
                                    println!(
                                        "{}",
                                        load_tja_from_path(&path, None, &conf, None).unwrap()
                                    );
                                    paths.push(path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    paths
}
