use crate::conf::Conf;
use crate::i18n;
use crate::tja;

pub fn load_tja_from_path<P>(
    path: P,
    encoding: Option<&'static encoding_rs::Encoding>,
    conf: &Conf,
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
        let mut index_low = 0;
        let mut previous_character = ' ';
        let mut index_high = 0;
        let mut flag_comment = false;
        let mut key = "";
        let mut flag_command = false;
        let mut course = tja::course::Course::default();
        for (i, character) in text.char_indices() {
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
                    key = text.get(index_low..index_high).unwrap();
                } else {
                    value = text.get(index_low..index_high).unwrap();
                }
                match key {
                    "TITLE" => {
                        chart.meta.title.set(
                            value,
                            *conf.locales.first().unwrap_or(&i18n::Locale::default()),
                        );
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
                            chart.meta.bpm = value;
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
                        if let Some(difficulty) = tja::course::meta::Course::from_str(value) {
                            course.meta.course = difficulty;
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
                    // TODO: add #START
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
                    }
                    // TODO: add other commands
                    _ => {
                        if !key.is_empty() {
                            println!("{:?}: {:?}", key, value);
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
                        key = text.get(index_low..i).unwrap();
                        index_low = i + 1;
                    }
                    '#' => {
                        index_low = i;
                        flag_command = true;
                    }
                    ' ' => {
                        if flag_command {
                            key = text.get(index_low..i).unwrap();
                            index_low = i + 1;
                            flag_command = false;
                        }
                    }
                    // TODO: add notes and comma
                    _ => {}
                }
            }
            previous_character = character
        }
        Some(chart)
    } else {
        None
    }
}
