pub mod course;
pub mod exam;
pub mod style;
pub use {course::Course, exam::Exam, style::Style};

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
            course: Course::default(),
            level: 0,
            balloon: Vec::new(),
            scoreinit: 0,   // TODO: figure out the true default value
            scorediff: 100, // TODO: figure out the true default value
            style: Style::default(),
            exam1: None,
            exam2: None,
            exam3: None,
        }
    }
}

impl std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "COURSE:{:?}", self.course)?;
        writeln!(f, "LEVEL:{}", self.level)?;
        if !self.balloon.is_empty() {
            write!(f, "BALLOON:{}", self.balloon[0])?;
            for count in &self.balloon[1..] {
                write!(f, ",{}", count)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "SCOREDIFF:{}", self.scorediff)?;
        writeln!(f, "SCOREINIT:{}", self.scoreinit)?;
        write!(f, "STYLE:{:?}", self.style)?;
        if let Some(exam) = &self.exam1 {
            write!(f, "\nEXAM1:{:?}", exam)?;
        }
        if let Some(exam) = &self.exam2 {
            write!(f, "\nEXAM2:{:?}", exam)?;
        }
        if let Some(exam) = &self.exam3 {
            write!(f, "\nEXAM3:{:?}", exam)?;
        }
        Ok(())
    }
}
