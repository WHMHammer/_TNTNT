mod conf;
mod i18n;
//mod loaders;
mod player;
mod tja;

fn main() {
    // all codes here are purely for testing purposes; there is no runnable application yet
    // TODO: test branching
    let tja_path = std::path::Path::new("Chun Jie Xu Qu/Chun Jie Xu Qu.tja");
    //let tja_path = std::path::Path::new("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja");

    let conf = conf::Conf::default();
    let chart =
        tja::Chart::parse_from_path(tja_path, None, &conf, Some(&"box.def Genre".to_string()))
            .unwrap();

    let sounds = player::resources::Sounds::load_from_directory("snd");
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    if let Some(wave) = &chart.meta.wave {
        if let Some(directory) = tja_path.parent() {
            player::resources::sound::play_audio_from_path(&stream_handle, directory.join(wave));
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
    for tja::course::Event { offset, event_type } in &chart.oni_course.as_ref().unwrap().p0 {
        match event_type {
            tja::course::event::Don | tja::course::event::DON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < *offset {}
                sounds.play_don(&stream_handle);
            }
            tja::course::event::Ka | tja::course::event::KA => {
                while t.elapsed().as_millis() as f64 / 1000.0 < *offset {}
                sounds.play_ka(&stream_handle);
            }
            tja::course::event::Balloon | tja::course::event::BALLOON => {
                flag_balloon = true;
            }
            tja::course::event::End => loop {
                let millis = t.elapsed().as_millis();
                if millis as f64 / 1000.0 >= offset - 0.1 {
                    if flag_balloon {
                        sounds.play_don(&stream_handle);
                        flag_balloon = false;
                    }
                    break;
                }
                if millis % 100 == 0 {
                    sounds.play_don(&stream_handle);
                }
            },
            _ => {}
        }
        println!("{:?}", event_type);
    }
    std::thread::sleep(std::time::Duration::from_secs(15));
}
