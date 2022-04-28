mod conf;
mod i18n;
mod loaders;
mod tja;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let conf = conf::Conf::default();
    let chart = tja::Chart::parse_from_path(
        "Chun Jie Xu Qu/Chun Jie Xu Qu.tja",
        None,
        &conf,
        Some(&"box.def Genre".to_string()),
    )
    .unwrap();
    //println!("{:#?}", chart);
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
    let mut previous_event_type = tja::course::event::Empty;
    for tja::course::Event { offset, event_type } in chart.oni_course.unwrap().p1 {
        match event_type {
            tja::course::event::Don | tja::course::event::DON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < offset {}
                stream_handle
                    .play_raw(
                        Decoder::new(BufReader::new(File::open("snd/don.ogg").unwrap()))
                            .unwrap()
                            .convert_samples(),
                    )
                    .unwrap();
            }
            tja::course::event::Ka | tja::course::event::KA => {
                while t.elapsed().as_millis() as f64 / 1000.0 < offset {}
                stream_handle
                    .play_raw(
                        Decoder::new(BufReader::new(File::open("snd/ka.ogg").unwrap()))
                            .unwrap()
                            .convert_samples(),
                    )
                    .unwrap();
            }
            tja::course::event::Drumroll
            | tja::course::event::DRUMROLL
            | tja::course::event::Balloon
            | tja::course::event::BALLOON => {}
            tja::course::event::End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= offset - 0.1 {
                    match previous_event_type {
                        tja::course::event::Balloon | tja::course::event::BALLOON => {
                            stream_handle
                                .play_raw(
                                    Decoder::new(BufReader::new(
                                        File::open("snd/balloon.ogg").unwrap(),
                                    ))
                                    .unwrap()
                                    .convert_samples(),
                                )
                                .unwrap();
                        }
                        _ => {}
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
        previous_event_type = event_type;
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
}
