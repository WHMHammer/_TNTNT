mod application;
mod i18n;
mod tja;
use application::audio::Audio;
use std::time::{Duration, Instant};
use tja::course::meta::course_name::CourseName;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    let conf = application::conf::Conf::default();
    let resources = application::resources::Resources::load_from_directory("System/SimpleStyle/"); // TJAPlayer3-style resources
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let tja_path = std::path::Path::new("Chun Jie Xu Qu/Chun Jie Xu Qu.tja");
    let directory = tja_path.parent().unwrap();
    let chart =
        tja::Chart::parse_from_path(tja_path, None, &conf, Some(&"box.def Genre".to_string()))
            .unwrap();
    let course = chart.oni_course.as_ref().unwrap();
    let events = &course.p0;
    println!("{:?}", course.meta);

    if let CourseName::Dan = course.meta.course {
    } else {
        sink.append(
            Audio::load_from_path(directory.join(chart.meta.wave.as_ref().unwrap()))
                .unwrap()
                .decoder()
                .unwrap(),
        );
    }
    let mut flag_balloon = false;
    let mut flag_rolled = false;
    let mut t;
    if chart.meta.offset < 0.0 {
        t = Instant::now() + Duration::from_secs_f64(-chart.meta.offset);
        while t.elapsed().as_nanos() <= 0 {}
    } else {
        t = Instant::now() - Duration::from_secs_f64(chart.meta.offset);
    }
    for event in events {
        use application::resources::sounds::Sound;
        use tja::course::event::EventType::*;
        match &event.event_type {
            Don | DON | DualDON | ADLIB | PURPLE => {
                while t.elapsed().as_secs_f64() < event.offset {}
                resources.sounds.play(&stream_handle, Sound::Don);
            }
            Ka | KA | DualKA => {
                while t.elapsed().as_secs_f64() < event.offset {}
                resources.sounds.play(&stream_handle, Sound::Ka);
            }
            Drumroll | DRUMROLL => while t.elapsed().as_secs_f64() < event.offset {},
            Balloon | BALLOON => {
                while t.elapsed().as_secs_f64() < event.offset {}
                flag_balloon = true;
            }
            End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= event.offset {
                    if flag_balloon {
                        resources.sounds.play(&stream_handle, Sound::Balloon);
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 90 == 0 {
                    if !flag_rolled {
                        resources.sounds.play(&stream_handle, Sound::Don);
                        flag_rolled = true;
                    }
                } else {
                    flag_rolled = false;
                }
            },
            BRANCH(branches) => {
                use tja::course::event::branch::Thresholds::*;
                for event in match branches.thresholds {
                    r(_, _) => {
                        println!("#M");
                        &branches.m
                    }
                    p(e_threshold, m_threshold) => {
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
                            while t.elapsed().as_secs_f64() < event.offset {}
                            resources.sounds.play(&stream_handle, Sound::Don);
                        }
                        Ka | KA => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            resources.sounds.play(&stream_handle, Sound::Ka);
                        }
                        Drumroll | DRUMROLL => while t.elapsed().as_secs_f64() < event.offset {},
                        Balloon | BALLOON => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            flag_balloon = true;
                        }
                        End => loop {
                            let millis = t.elapsed().as_millis();
                            if millis as f64 / 1000.0 >= event.offset {
                                if flag_balloon {
                                    resources.sounds.play(&stream_handle, Sound::Balloon);
                                    flag_balloon = false;
                                }
                                break;
                            }
                            if millis % 90 == 0 {
                                if !flag_rolled {
                                    resources.sounds.play(&stream_handle, Sound::Don);
                                    flag_rolled = true;
                                }
                            } else {
                                flag_rolled = false;
                            }
                        },
                        _ => {}
                    }
                    println!("{:?}", event);
                }
                continue;
            }
            NEXTSONG(nextsong) => {
                sink.sleep_until_end();
                sink.append(
                    Audio::load_from_path(directory.join(&nextsong.wave))
                        .unwrap()
                        .decoder()
                        .unwrap(),
                );
                t = Instant::now();
            }
            _ => {}
        }
        println!("{:?}", event);
    }
    sink.sleep_until_end();
}
