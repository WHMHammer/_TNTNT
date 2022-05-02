mod conf;
mod i18n;
mod player;
mod tja;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    let tja_path = std::path::Path::new("Chun Jie Xu Qu/Chun Jie Xu Qu.tja");
    //let tja_path = std::path::Path::new("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja");

    let conf = conf::Conf::default();
    let chart =
        tja::Chart::parse_from_path(tja_path, None, &conf, Some(&"box.def Genre".to_string()))
            .unwrap();

    let sounds = player::resources::Sounds::load_from_directory("System/Switch-Style/Sounds/"); // TJAPlayer3-style resources
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    if let Some(wave) = &chart.meta.wave {
        if let Some(directory) = tja_path.parent() {
            if let Ok(file) = std::fs::File::open(directory.join(wave)) {
                if let Ok(decoder) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                    sink.append(decoder)
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
    for event in &chart.oni_course.as_ref().unwrap().p0 {
        match &event.event_type {
            tja::course::event::Don | tja::course::event::DON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                sounds.play(&stream_handle, player::resources::sound::Don);
            }
            tja::course::event::Ka | tja::course::event::KA => {
                while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                sounds.play(&stream_handle, player::resources::sound::Ka);
            }
            tja::course::event::Balloon | tja::course::event::BALLOON => {
                flag_balloon = true;
            }
            tja::course::event::End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= event.offset - 0.1 {
                    if flag_balloon {
                        sounds.play(&stream_handle, player::resources::sound::Balloon);
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 100 == 0 {
                    sounds.play(&stream_handle, player::resources::sound::Don);
                }
            },
            tja::course::event::BRANCH(branches) => {
                for event in match branches.thresholds {
                    tja::course::event::branch::Thresholds::r(_, _) => {
                        println!("#M");
                        &branches.m
                    }
                    tja::course::event::branch::Thresholds::p(e_threshold, m_threshold) => {
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
                        tja::course::event::Don | tja::course::event::DON => {
                            while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                            sounds.play(&stream_handle, player::resources::sound::Don);
                        }
                        tja::course::event::Ka | tja::course::event::KA => {
                            while t.elapsed().as_millis() as f64 / 1000.0 < event.offset {}
                            sounds.play(&stream_handle, player::resources::sound::Ka);
                        }
                        tja::course::event::Balloon | tja::course::event::BALLOON => {
                            flag_balloon = true;
                        }
                        tja::course::event::End => loop {
                            let millis = t.elapsed().as_millis();
                            if millis as f64 / 1000.0 >= event.offset - 0.1 {
                                if flag_balloon {
                                    sounds.play(&stream_handle, player::resources::sound::Balloon);
                                    flag_balloon = false;
                                }
                                break;
                            }
                            if millis % 100 == 0 {
                                sounds.play(&stream_handle, player::resources::sound::Don);
                            }
                        },
                        _ => {}
                    }
                    println!("{:?}", event);
                }
                continue;
            }
            _ => {}
        }
        println!("{:?}", event);
    }
    sink.sleep_until_end();
}
