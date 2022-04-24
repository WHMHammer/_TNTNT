pub mod course;
pub mod meta;

#[derive(Debug, Default)]
pub struct TjaChart {
    pub meta: meta::Meta,
    pub easy_course: Option<course::Course>,
    pub normal_course: Option<course::Course>,
    pub hard_course: Option<course::Course>,
    pub oni_course: Option<course::Course>,
    pub edit_course: Option<course::Course>,
    pub dan_course: Option<course::Course>,
    pub tower_course: Option<course::Course>,
}
