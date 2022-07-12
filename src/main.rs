#![allow(unused)]
// all codes here are purely for testing purposes; there is no runnable application yet
mod audio;
mod configurations;
mod test;
mod visualizer;
use configurations::resources::images::{LaneConfigurations, NotesConfigurations};

use test::play;

fn main() {
    use tja::event::event_type::EventType;
    let chart = tja::Chart::load(
        "charts/恋のうた (feat. 由崎司)/恋のうた (feat. 由崎司).tja",
        None,
    )
    .unwrap();
    let events = chart.oni_course.unwrap().p0;

    let notes_configurations = NotesConfigurations::load("resources/images/notes/notes.json");
    let lane_configurations = LaneConfigurations::load("resources/images/lane/lane.json");
    let visualizer_ui_layout =
        visualizer::VisualizerUiLayout::load("resources/images/visualizer.xml", 0, 0);

    let measure = visualizer_ui_layout.lane(1.0, visualizer::Branch::N, &lane_configurations);
    measure.unwrap().save("out.png").unwrap();
    let don = image::open("resources/images/notes/don.png").unwrap();
    let ka = image::open("resources/images/notes/ka.png").unwrap();
    let barline = image::open("resources/images/lane/barline.png").unwrap();

    // TODO: visualize

    play("charts/Chun Jie Xu Qu/Chun Jie Xu Qu.tja"); // example tja chart I wrote
    //play("charts/恋のうた (feat. 由崎司)/恋のうた (feat. 由崎司).tja"); // another example tja chart I wrote
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\11 Dan Dojo\\01 Nijiiro 2021 Version\\01 Nijiiro 2021 Fifth Kyuu\\Nijiiro 2021 Fifth Kyuu.tja"); // Dan
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\10 Taiko Towers\\Taiko Tower 10 Kara-kuchi\\Taiko Tower 10 Kara-kuchi.tja"); // Tower
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\07 Game Music\\Anzu no Uta\\Anzu no Uta.tja"); // branching (with branches impossible to reach) and #BARLINE[OFF/ON]
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja"); // DOUBLE
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\ANTI-HERO\\ANTI-HERO.tja"); // LYRIC
}
