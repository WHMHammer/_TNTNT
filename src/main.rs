mod audio;
use std::time::{Duration, Instant};

use audio::Device;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    let tja_path = std::path::Path::new("Chun Jie Xu Qu/Chun Jie Xu Qu.tja");
    let directory = tja_path.parent().unwrap();
    let chart = tja::Chart::parse_from_path(tja_path, None).unwrap();
    let course = chart.oni_course.as_ref().unwrap();
    let events = &course.p0;
    println!("{:?}", chart.meta);
    println!("{:?}", course.meta);

    let sdl = sdl2::init().unwrap();
    let audio_subsystem = sdl.audio().unwrap();

    let mut music = Device::from_path(
        directory.join(chart.meta.wave.as_ref().unwrap()),
        &audio_subsystem,
    )
    .unwrap();
    let mut don =
        Device::from_path("System/SimpleStyle/Sounds/Taiko/dong.ogg", &audio_subsystem).unwrap();
    let mut ka =
        Device::from_path("System/SimpleStyle/Sounds/Taiko/ka.ogg", &audio_subsystem).unwrap();
    let mut balloon =
        Device::from_path("System/SimpleStyle/Sounds/balloon.ogg", &audio_subsystem).unwrap();

    let mut flag_balloon = false;
    let mut flag_rolled = false;
    let mut t;
    music.play();
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
                don.play();
            }
            Ka | KA | DualPlayerKa => {
                while t.elapsed().as_secs_f64() < event.offset {}
                ka.play()
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
                        balloon.play();
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 90 == 0 {
                    if !flag_rolled {
                        don.play();
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
                            don.play();
                        }
                        Ka | KA => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            ka.play();
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
                                    balloon.play();
                                    flag_balloon = false;
                                }
                                break;
                            }
                            if millis % 90 == 0 {
                                if !flag_rolled {
                                    don.play();
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
                while t.elapsed() < music.duration() {}
                music =
                    Device::from_path(directory.join(&nextsong.wave), &audio_subsystem).unwrap();
                t = Instant::now();
            }
            _ => {}
        }
        println!("{:?}", event);
    }
    while t.elapsed() < music.duration() {}
}
