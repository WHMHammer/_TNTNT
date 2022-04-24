pub mod course;
pub mod exam;
pub mod style;
pub use {
    course::Course, exam::Exam, style::Style,
};

#[derive(Debug)]
pub struct Meta {
    pub course: Course,
    pub level: u8,
    pub balloon: Vec<u8>,
    pub scoreinit: u32,
    pub scorediff: u32,
    pub style: Style,
    pub exam1: Option<Exam>,
    pub exam2: Option<Exam>,
    pub exam3: Option<Exam>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            course: Course::Oni,
            level: 0,
            balloon: Vec::new(),
            scoreinit: 0,
            scorediff: 100,
            style: Style::Single,
            exam1: None,
            exam2: None,
            exam3: None,
        }
    }
}
