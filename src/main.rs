#![allow(unused)]
// all codes here are purely for testing purposes; there is no runnable application yet
mod audio;
mod configurations;
mod test;
mod visualizer;
use configurations::resources::images::{LaneConfigurations, NotesConfigurations};

use test::play;
use tja::event;

fn main() {
    /*use tja::event::event_type::EventType;
    let chart = tja::Chart::load(
        "charts/恋のうた (feat. 由崎司)/恋のうた (feat. 由崎司).tja",
        None,
    )
    .unwrap();
    let events = &chart.oni_course.unwrap().p0;

    let notes_configurations = NotesConfigurations::load("resources/images/notes/notes.json");
    let lane_configurations = LaneConfigurations::load("resources/images/lane/lane.json");
    let visualizer_ui_layout = visualizer::VisualizerUiLayout::load(
        "resources/images/visualizer.xml",
        0,
        0,
        &lane_configurations,
        &notes_configurations,
    );

    let measure = visualizer_ui_layout.lane(1.0, visualizer::Branch::N);
    measure.unwrap().save("out.png").unwrap();
    let don = image::open("resources/images/notes/don.png").unwrap();
    let ka = image::open("resources/images/notes/ka.png").unwrap();
    let barline = image::open("resources/images/lane/barline.png").unwrap();

    let mut index_low = 0;
    let mut index_high = 1;
    let mut position_low = 0.0;
    let mut position_high;
    //let mut measures = Vec::new();
    while index_high < events.len() {
        while index_high < events.len() {
            if let EventType::Barline = events[index_high].event_type {
                break;
            }
            index_high += 1;
        }
        position_high = events[index_high].position_offset;
        let mut measure =
            visualizer_ui_layout.lane(position_high - position_low, visualizer::Branch::N);
        while index_low <= index_high {
            use EventType::*;
        }
    }*/

    play("D:\\Gaming\\Taiko\\Favorites\\00 WHMHammer自制\\Rush E\\Rush E (Impossible).tja"); // LYRIC
    //play("charts/Chun Jie Xu Qu/Chun Jie Xu Qu.tja"); // example tja chart I wrote
    //play("charts/恋のうた (feat. 由崎司)/恋のうた (feat. 由崎司).tja"); // another example tja chart I wrote
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\11 Dan Dojo\\01 Nijiiro 2021 Version\\01 Nijiiro 2021 Fifth Kyuu\\Nijiiro 2021 Fifth Kyuu.tja"); // Dan
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\10 Taiko Towers\\Taiko Tower 10 Kara-kuchi\\Taiko Tower 10 Kara-kuchi.tja"); // Tower
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\07 Game Music\\Anzu no Uta\\Anzu no Uta.tja"); // branching (with branches impossible to reach) and #BARLINE[OFF/ON]
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\Natsu Matsuri\\Natsu Matsuri -New Audio-.tja"); // DOUBLE
    //play("D:\\Gaming\\Taiko\\Official Songs 20220423\\01 Pop\\ANTI-HERO\\ANTI-HERO.tja"); // LYRIC
}
