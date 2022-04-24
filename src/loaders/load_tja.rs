use crate::i18n;
use crate::tja;

pub fn load_tja_from_string(string: String, genre: Option<&String>) -> tja::TjaChart {
    let mut chart = tja::TjaChart::default();
    chart.meta.genre = genre.map(String::clone);
    let mut index_low = 0;
    let mut key: Option<&str> = None;
    let mut flag_first_slash = false;
    let mut flag_comment = false;
    let mut flag_command = false;
    let mut flag_course_started = false;
    let mut note_count = 0;
    for (i, character) in string.char_indices() {
        if character == '\n' {
            if !flag_comment {
                if let (Some(k), Some(v)) = (key, string.get(index_low..i)) {
                    let v = v.trim();
                    match k.trim() {
                        "TITLE" => {
                            chart.meta.title.set_default(v);
                        }
                        "TITLEEN" => {
                            chart.meta.title.set(v, i18n::en_US);
                        }
                        "TITLECN" => {
                            chart.meta.title.set(v, i18n::zh_CN);
                        }
                        "SUBTITLE" => {
                            chart.meta.subtitle.set_default(v);
                        }
                        "SUBTITLEEN" => {
                            chart.meta.subtitle.set(v, i18n::en_US);
                        }
                        "SUBTITLECN" => {
                            chart.meta.subtitle.set(v, i18n::zh_CN);
                        }
                        "BPM" => {
                            if let Ok(value) = v.parse() {
                                chart.meta.bpm = value;
                            }
                        }
                        "WAVE" => {
                            chart.meta.wave = Some(v.to_string());
                        }
                        "OFFSET" => {
                            if let Ok(value) = v.parse() {
                                chart.meta.offset = value;
                            }
                        }
                        "DEMOSTART" => {
                            if let Ok(value) = v.parse() {
                                chart.meta.demostart = value;
                            }
                        }
                        "GENRE" => {
                            chart.meta.genre = Some(v.to_string());
                        }
                        "SCOREMODE" => {
                            if let Some(mode) = tja::meta::ScoreMode::from_str(v) {
                                chart.meta.scoremode = mode;
                            }
                        }
                        "LIFE" => {
                            if let Ok(value) = v.parse() {
                                chart.meta.life = value;
                            }
                        }
                        "BGMOVIE" => {
                            chart.meta.bgmovie = Some(v.to_string());
                        }
                        // TODO: add course-specific meta
                        // TODO: add commands with parameters
                        _ => {}
                    }
                } else {
                    if let Some(line) = string.get(index_low..i) {
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
                    key = string.get(index_low..i);
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
                        if let Some(measure) = string.get(index_low..i) {
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
    chart
}
