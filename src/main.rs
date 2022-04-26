mod conf;
mod i18n;
mod loaders;
mod tja;

fn main() {
    let mut conf = conf::Conf::default();
    //conf.locales = vec![i18n::Locale::zh_CN];
    let chart = tja::Chart::from_path(
        "/mnt/d/Gaming/Taiko/Favorites/00 WHMHammer自制/Chun Jie Xu Qu/Chun Jie Xu Qu.tja",
        None,
        &conf,
        Some(&"box.def Genre".to_string()),
    )
    .unwrap();
    println!("{:#?}", chart);

    //loaders::get_all_tja_paths("/mnt/d/Gaming/Taiko/Favorites");
}
