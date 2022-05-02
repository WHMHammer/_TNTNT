mod i18n;
mod player;
mod tja;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    //let tja_path = std::path::Path::new("Chun Jie Xu Qu/Chun Jie Xu Qu.tja");
    //let tja_path = std::path::Path::new("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja");
    let tja_path = std::path::Path::new("D:\\Gaming\\Taiko\\Official Songs 20220423\\11 Dan Dojo\\01 Nijiiro 2021 Version\\01 Nijiiro 2021 Fifth Kyuu\\Nijiiro 2021 Fifth Kyuu.tja");

    let conf = player::Conf::default();
    let chart =
        tja::Chart::parse_from_path(tja_path, None, &conf, Some(&"box.def Genre".to_string()))
            .unwrap();
    let course = chart.dan_course.as_ref().unwrap();

    let sounds = player::resources::Sounds::load_from_directory("System/Switch-Style/Sounds/"); // TJAPlayer3-style resources
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    if course.meta.course != tja::course::meta::Course::Dan {
        if let Some(wave) = &chart.meta.wave {
            if let Some(directory) = tja_path.parent() {
                if let Ok(file) = std::fs::File::open(directory.join(wave)) {
                    if let Ok(decoder) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                        sink.append(decoder);
                    }
                }
            }
        }
    }

    let mut t = std::time::Instant::now();
    if chart.meta.offset < 0.0 {
        let offset = std::time::Duration::from_secs_f64(-chart.meta.offset);
        t += offset;
        std::thread::sleep(offset);
    } else {
        t -= std::time::Duration::from_secs_f64(chart.meta.offset);
    }
    let mut flag_balloon = false;
    for event in &course.p0 {
        use tja::course::event::*;
        match &event.event_type {
            Don | DON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                sounds.play(&stream_handle, player::resources::sounds::Don);
            }
            Ka | KA => {
                while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                sounds.play(&stream_handle, player::resources::sounds::Ka);
            }
            Balloon | BALLOON => {
                flag_balloon = true;
            }
            End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= event.offset - 0.1 {
                    if flag_balloon {
                        sounds.play(&stream_handle, player::resources::sounds::Balloon);
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 100 == 0 {
                    sounds.play(&stream_handle, player::resources::sounds::Don);
                }
            },
            BRANCH(branches) => {
                for event in match branches.thresholds {
                    branch::Thresholds::r(_, _) => {
                        println!("#M");
                        &branches.m
                    }
                    branch::Thresholds::p(e_threshold, m_threshold) => {
                        if m_threshold <= 100.0 {
                            println!("#M");
                            &branches.m
                        } else if e_threshold <= 100.0 {
                            println!("#E");
                            &branches.e
                        } else {
                            println!("#N");
                            &branches.n
                        }
                    }
                } {
                    match event.event_type {
                        Don | DON => {
                            while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                            sounds.play(&stream_handle, player::resources::sounds::Don);
                        }
                        Ka | KA => {
                            while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                            sounds.play(&stream_handle, player::resources::sounds::Ka);
                        }
                        Balloon | BALLOON => {
                            flag_balloon = true;
                        }
                        End => loop {
                            let millis = t.elapsed().as_millis();
                            if millis as f64 / 1000.0 >= event.offset - 0.1 {
                                if flag_balloon {
                                    sounds.play(&stream_handle, player::resources::sounds::Balloon);
                                    flag_balloon = false;
                                }
                                break;
                            }
                            if millis % 100 == 0 {
                                sounds.play(&stream_handle, player::resources::sounds::Don);
                            }
                        },
                        _ => {}
                    }
                    println!("{:?}", event);
                }
                continue;
            }
            NEXTSONG(nextsong) => {
                if let Some(directory) = tja_path.parent() {
                    if let Ok(file) = std::fs::File::open(directory.join(&nextsong.wave)) {
                        if let Ok(decoder) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                            sink.sleep_until_end();
                            sink.append(decoder);
                            t = std::time::Instant::now();
                        }
                    }
                }
            }
            _ => {}
        }
        println!("{:?}", event);
    }
    sink.sleep_until_end();
}
