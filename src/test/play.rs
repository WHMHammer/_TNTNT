use std::time::{Duration, Instant};
use tja::course::meta::difficulty::Difficulty;

pub fn play<P>(path: P)
where
    P: AsRef<std::path::Path>,
{
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let don = crate::audio::Audio::load("resources/audios/taiko/don.wav").unwrap();
    let ka = crate::audio::Audio::load("resources/audios/taiko/ka.wav").unwrap();
    let balloon = crate::audio::Audio::load("resources/audios/taiko/balloon.wav").unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let tja_path = path.as_ref();
    let directory = tja_path.parent().unwrap();
    let chart = tja::Chart::load(tja_path, None).unwrap();
    let course = chart.dan_course.as_ref().unwrap_or_else(|| {
        chart.tower_course.as_ref().unwrap_or_else(|| {
            chart.edit_course.as_ref().unwrap_or_else(|| {
                chart.oni_course.as_ref().unwrap_or_else(|| {
                    chart.hard_course.as_ref().unwrap_or_else(|| {
                        chart
                            .normal_course
                            .as_ref()
                            .unwrap_or_else(|| chart.easy_course.as_ref().unwrap())
                    })
                })
            })
        })
    });
    let events = if !course.p2.is_empty() {
        if course.p1.is_empty() {
            panic!(); // only one player's chart is presented while STYLE:DOUBLE
        }
        &course.p2
    } else if !course.p1.is_empty() {
        panic!(); // only one player's chart is presented while STYLE:DOUBLE
    } else {
        &course.p0
    };
    println!("{:?}", course.meta);

    if let Difficulty::Dan = course.meta.course {
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
        use tja::event::EventType::*;
        match &event.event_type {
            Don | BigDon | DualPlayerDon | ADLIB | Purple => {
                while t.elapsed().as_secs_f64() < event.offset {}
                don.play(&stream_handle).unwrap();
            }
            Ka | BigKa | DualPlayerKa => {
                while t.elapsed().as_secs_f64() < event.offset {}
                ka.play(&stream_handle).unwrap();
            }
            Drumroll | BigDrumroll => while t.elapsed().as_secs_f64() < event.offset {},
            Balloon | BigBalloon => {
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
            Branch(branches) => {
                use tja::event::branch::Thresholds::*;
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
                        Don | BigDon => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            don.play(&stream_handle).unwrap();
                        }
                        Ka | BigKa => {
                            while t.elapsed().as_secs_f64() < event.offset {}
                            ka.play(&stream_handle).unwrap();
                        }
                        Drumroll | BigDrumroll => while t.elapsed().as_secs_f64() < event.offset {},
                        Balloon | BigBalloon => {
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
            NextSong(next_song) => {
                sink.sleep_until_end();
                sink.append(
                    rodio::decoder::Decoder::new(
                        std::fs::File::open(directory.join(&next_song.wave)).unwrap(),
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
