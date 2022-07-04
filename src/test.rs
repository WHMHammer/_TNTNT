use std::time::{Duration, Instant};
use tja::course::meta::course_name::CourseName;

pub fn play<P>(path: P)
where
    P: AsRef<std::path::Path>,
{
    // all codes here are purely for testing purposes; there is no runnable application yet
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let don = crate::audio::Audio::load("resources/audios/taiko/don.wav").unwrap();
    let ka = crate::audio::Audio::load("resources/audios/taiko/ka.wav").unwrap();
    let balloon = crate::audio::Audio::load("resources/audios/taiko/balloon.wav").unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let tja_path = path.as_ref();
    let directory = tja_path.parent().unwrap();
    let chart = tja::Chart::parse_from_path(tja_path, None).unwrap();
    let course = chart
        .oni_course
        .as_ref()
        .unwrap_or_else(|| chart.dan_course.as_ref().unwrap());
    let events = &course.p0;
    println!("{:?}", course.meta);

    if let CourseName::Dan = course.meta.course {
    } else {
        sink.append(
            rodio::decoder::Decoder::new(
                std::fs::File::open(directory.join(chart.meta.wave.as_ref().unwrap())).unwrap(),
            )
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
        use tja::course::event::EventType::*;
        match &event.event_type {
            Don | DON | DualPlayerDON | ADLIB | PURPLE => {
                while t.elapsed().as_secs_f64() < event.offset {}
                don.play(&stream_handle).unwrap();
            }
            Ka | KA | DualPlayerKa => {
                while t.elapsed().as_secs_f64() < event.offset {}
                ka.play(&stream_handle).unwrap();
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
                        balloon.play(&stream_handle).unwrap();
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 50 == 0 {
                    if !flag_rolled {
                        don.play(&stream_handle).unwrap();
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
                            don.play(&stream_handle).unwrap();
                        }
                        Ka | KA => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            ka.play(&stream_handle).unwrap();
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
                                    balloon.play(&stream_handle).unwrap();
                                    flag_balloon = false;
                                }
                                break;
                            }
                            if millis % 50 == 0 {
                                if !flag_rolled {
                                    don.play(&stream_handle).unwrap();
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
                    rodio::decoder::Decoder::new(
                        std::fs::File::open(directory.join(&nextsong.wave)).unwrap(),
                    )
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
