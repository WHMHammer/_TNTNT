mod conf;
mod i18n;
mod loaders;
mod tja;

fn main() {
    let conf = conf::Conf::default();
    let chart = tja::Chart::parse_from_path(
        "/mnt/d/Gaming/Taiko/Favorites/00 WHMHammer自制/Chun Jie Xu Qu/Chun Jie Xu Qu.tja",
        //"/mnt/d/Gaming/Taiko/Official Songs 20220423/07 Game Music/Anzu no Uta/Anzu no Uta.tja",
        None,
        &conf,
        Some(&"box.def Genre".to_string()),
    )
    .unwrap();
    println!("{:#?}", chart);
    /*let t = std::time::Instant::now();
    for tja::course::Event { offset, event_type } in chart.oni_course.unwrap().p1 {
        match event_type {
            tja::course::event::Don
            | tja::course::event::Ka
            | tja::course::event::DON
            | tja::course::event::KA
            | tja::course::event::Drumroll
            | tja::course::event::DRUMROLL
            | tja::course::event::Balloon
            | tja::course::event::End
            | tja::course::event::BALLOON => {
                while t.elapsed().as_millis() as f64 / 1000.0 < offset {}
                println!("{:?}", event_type);
            }
            _ => {}
        }
    }*/

    let (_stream, _stream_handle) = rodio::OutputStream::try_default().unwrap();

    loaders::get_all_tja_paths("/mnt/d/Gaming/Taiko/Favorites");
}
