mod conf;
mod i18n;
mod loaders;
mod tja;

fn main() {
    let mut conf = conf::Conf::default();
    conf.locales = vec![];
    let chart = loaders::load_tja_from_path(
        "Chun Jie Xu Qu.tja",
        None,
        &conf,
        Some(&"box.def Genre".to_string()),
    )
    .unwrap();
    println!("{:#?}", chart);
}
