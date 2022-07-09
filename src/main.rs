// all codes here are purely for testing purposes; there is no runnable application yet
mod audio;
mod test;
use image::{
    imageops::{overlay, replace, resize, Lanczos3},
    RgbaImage,
};
use serde::{Deserialize, Serialize};
use test::play;

enum Branch {
    N,
    E,
    M,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct NoteConfig {
    center_x: Option<u32>,
    center_y: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct LaneConfig {
    width: Option<u32>,
    height: Option<u32>,
    n_center_y: Option<u32>,
    e_center_y: Option<u32>,
    m_center_y: Option<u32>,
    don: Option<NoteConfig>,
    ka: Option<NoteConfig>,
    big_don: Option<NoteConfig>,
    big_ka: Option<NoteConfig>,
    drumroll_head: Option<NoteConfig>,
    drumroll_body: Option<NoteConfig>,
    drumroll_tail: Option<NoteConfig>,
    big_drumroll_head: Option<NoteConfig>,
    big_drumroll_body: Option<NoteConfig>,
    big_drumroll_tail: Option<NoteConfig>,
    balloon_head: Option<NoteConfig>,
    balloon_tail: Option<NoteConfig>,
    big_balloon_head: Option<NoteConfig>,
    big_balloon_tail: Option<NoteConfig>,
    mine: Option<NoteConfig>,
    purple: Option<NoteConfig>,
}

impl LaneConfig {
    fn load<P>(path: P, width: u32, height: u32) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut config: Self = if let Ok(file) = std::fs::File::open(path) {
            quick_xml::de::from_reader(std::io::BufReader::new(file)).unwrap_or_default()
        } else {
            Default::default()
        };
        println!("{:#?}", &config);
        if config.width.is_none() {
            config.width = Some(width)
        }
        if config.height.is_none() {
            config.height = Some(height)
        }
        config
    }

    fn generate_measure(&self, branch: Branch) -> RgbaImage {
        let n = image::open("resources/images/lane/n.png").unwrap();
        let n = resize(&n, self.width.unwrap(), n.height(), Lanczos3);
        let mut canvas = RgbaImage::new(self.width.unwrap(), self.height.unwrap());
        replace(&mut canvas, &n, 0, 0);
        use Branch::*;
        match branch {
            N => {}
            E => {
                let e = image::open("resources/images/lane/e.png").unwrap();
                let e = resize(&e, self.width.unwrap(), e.height(), Lanczos3);
                overlay(&mut canvas, &e, 0, 0);
            }
            M => {
                let m = image::open("resources/images/lane/m.png").unwrap();
                let m = resize(&m, self.width.unwrap(), m.height(), Lanczos3);
                overlay(&mut canvas, &m, 0, 0);
            }
        }
        canvas
    }
}

fn main() {
    let lane_config = LaneConfig::load("resources/images/lane.xml", 1422, 264);
    lane_config
        .generate_measure(Branch::N)
        .save("n-measure.png")
        .unwrap();
    lane_config
        .generate_measure(Branch::E)
        .save("e-measure.png")
        .unwrap();
    lane_config
        .generate_measure(Branch::M)
        .save("m-measure.png")
        .unwrap();
    play("Chun Jie Xu Qu/Chun Jie Xu Qu.tja"); // example tja chart I wrote
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\11 Dan Dojo\\01 Nijiiro 2021 Version\\01 Nijiiro 2021 Fifth Kyuu\\Nijiiro 2021 Fifth Kyuu.tja"); // Dan
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\10 Taiko Towers\\Taiko Tower 10 Kara-kuchi\\Taiko Tower 10 Kara-kuchi.tja"); // Tower
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\07 Game Music\\Anzu no Uta\\Anzu no Uta.tja"); // branching (with branches impossible to reach)
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja"); // DOUBLE
}
