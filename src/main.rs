mod conf;
mod i18n;
mod loaders;
mod tja;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    let conf = conf::Conf::default();
    let chart = tja::Chart::parse_from_path(
        "Chun Jie Xu Qu/Chun Jie Xu Qu.tja",
        None,
        &conf,
        Some(&"box.def Genre".to_string()),
    )
    .unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle
        .play_raw(
            Decoder::new(BufReader::new(
                File::open("Chun Jie Xu Qu/Chun Jie Xu Qu.wav").unwrap(),
            ))
            .unwrap()
            .convert_samples(),
        )
        .unwrap();
    let t = std::time::Instant::now();
    let mut flag_balloon = false;
    for tja::course::Event { offset, event_type } in &chart.oni_course.as_ref().unwrap().p1 {
        match event_type {
            tja::course::event::Don | tja::course::event::DON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < *offset {}
                stream_handle
                    .play_raw(
                        Decoder::new(BufReader::new(File::open("snd/don.ogg").unwrap()))
                            .unwrap()
                            .convert_samples(),
                    )
                    .unwrap();
            }
            tja::course::event::Ka | tja::course::event::KA => {
                while t.elapsed().as_millis() as f64 / 1000.0 < *offset {}
                stream_handle
                    .play_raw(
                        Decoder::new(BufReader::new(File::open("snd/ka.ogg").unwrap()))
                            .unwrap()
                            .convert_samples(),
                    )
                    .unwrap();
            }
            tja::course::event::Balloon | tja::course::event::BALLOON => {
                flag_balloon = true;
            }
            tja::course::event::End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= offset - 0.1 {
                    if flag_balloon {
                        stream_handle
                            .play_raw(
                                Decoder::new(BufReader::new(
                                    File::open("snd/balloon.ogg").unwrap(),
                                ))
                                .unwrap()
                                .convert_samples(),
                            )
                            .unwrap();
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 100 == 0 {
                    stream_handle
                        .play_raw(
                            Decoder::new(BufReader::new(File::open("snd/don.ogg").unwrap()))
                                .unwrap()
                                .convert_samples(),
                        )
                        .unwrap();
                }
            },
            _ => {}
        }
        println!("{:?}", event_type);
    }
    println!("{:#?}", chart);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
