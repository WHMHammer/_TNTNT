use crate::configurations::resources::images::{LaneConfigurations, NotesConfigurations};
use image::{
    imageops::{overlay, replace, resize, Lanczos3},
    RgbaImage,
};

pub enum Branch {
    N,
    E,
    M,
}

#[derive(Default, serde::Deserialize)]
struct VisualizerUiLayoutXml {
    width: Option<u32>,
    height: Option<u32>,
    oy: Option<i64>,
}

#[derive(Debug)]
pub struct VisualizerUiLayout {
    width: u32,
    height: u32,
    oy: i64, // "o" means center
}

impl VisualizerUiLayout {
    pub fn load<P>(path: P, default_width: u32, default_height: u32) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let xml: VisualizerUiLayoutXml = if let Ok(file) = std::fs::File::open(path) {
            quick_xml::de::from_reader(std::io::BufReader::new(file)).unwrap_or_default()
        } else {
            Default::default()
        };
        let mut layout = Self {
            width: xml.width.unwrap_or(default_width),
            height: xml.height.unwrap_or(default_height),
            oy: 0,
        };
        layout.oy = xml.oy.unwrap_or((layout.height / 2) as i64);
        layout
    }

    pub fn lane(
        &self,
        width_scale: f64,
        branch: Branch,
        lane_configurations: &LaneConfigurations,
    ) -> Result<RgbaImage, ()> {
        let width = (width_scale * self.width as f64) as u32;
        let mut canvas = RgbaImage::new(width, self.height);
        let resource = image::open("resources/images/lane/lane.png").or(Err(()))?;
        let resource = resize(&resource, width, resource.height(), Lanczos3);
        replace(
            &mut canvas,
            &resource,
            0,
            self.oy
                - if let Some(lane) = &lane_configurations.lane {
                    lane.oy.unwrap_or((resource.height() / 2) as i64)
                } else {
                    (resource.height() / 2) as i64
                },
        );
        use Branch::*;
        match &branch {
            N => {}
            _ => {
                let resource = image::open(match &branch {
                    N => unreachable!(),
                    E => "resources/images/lane/e-branch.png",
                    M => "resources/images/lane/m-branch.png",
                })
                .or(Err(()))?;
                let resource = resize(&resource, canvas.width(), resource.height(), Lanczos3);
                overlay(
                    &mut canvas,
                    &resource,
                    0,
                    self.oy
                        - if let Some(lane) = match &branch {
                            N => unreachable!(),
                            E => &lane_configurations.e_branch,
                            M => &lane_configurations.m_branch,
                        } {
                            lane.oy.unwrap_or((resource.height() / 2) as i64)
                        } else {
                            (resource.height() / 2) as i64
                        },
                );
            }
        }
        Ok(canvas)
    }
}
