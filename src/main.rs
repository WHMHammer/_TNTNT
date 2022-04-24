mod i18n;
mod loaders;
mod tja;

fn main() {
    let chart = loaders::load_tja_from_string(
        loaders::load_text_from_path("Anzu no Uta.tja", None).unwrap(),
        None,
    );
    println!("{:#?}", chart);
}
