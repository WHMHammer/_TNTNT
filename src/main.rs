mod tja;

fn main() {
    println!("{:#?}", tja::Chart::from_path("Chun Jie Xu Qu.tja").unwrap());
}
