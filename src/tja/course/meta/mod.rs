pub mod course;
pub mod exam;
pub mod style;
use {course::Course, exam::Exam};

pub struct Meta {
    pub course: Course,
    pub level: u8,
    pub balloon: Vec<u8>,
    pub balloon_double: Vec<u8>,
    pub scoreinit: u32,
    pub scoreinit_double: u32,
    pub scorediff: u32,
    pub scorediff_double: u32,
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
            balloon_double: Vec::with_capacity(0),
            scoreinit: 0,          // TODO: figure out the true default value
            scoreinit_double: 0,   // TODO: figure out the true default value
            scorediff: 100,        // TODO: figure out the true default value
            scorediff_double: 100, // TODO: figure out the true default value
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
        if !self.balloon_double.is_empty() {
            write!(f, "BALLOON (Double):{}", self.balloon_double[0])?;
            for count in &self.balloon_double[1..] {
                write!(f, ",{}", count)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "SCOREDIFF:{}", self.scorediff)?;
        writeln!(f, "SCOREDIFF (Double):{}", self.scorediff_double)?;
        writeln!(f, "SCOREINIT:{}", self.scoreinit)?;
        write!(f, "SCOREINIT (Double):{}", self.scoreinit_double)?;
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
