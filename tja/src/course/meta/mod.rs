pub mod difficulty;
pub mod exam;
pub mod style;
use {difficulty::Difficulty, exam::Exam};

pub struct CourseMeta {
    pub course: Difficulty,
    pub level: u8,
    pub balloon: Vec<u8>,
    pub balloon_double: Vec<u8>,
    pub score_init: u32,
    pub score_init_double: u32,
    pub score_diff: u32,
    pub score_diff_double: u32,
    pub exam1: Option<Exam>,
    pub exam2: Option<Exam>,
    pub exam3: Option<Exam>,
}

impl Default for CourseMeta {
    fn default() -> Self {
        Self {
            course: Difficulty::default(),
            level: 0,
            balloon: Vec::new(),
            balloon_double: Vec::with_capacity(0),
            score_init: 0,          // TODO: figure out the true default value
            score_init_double: 0,   // TODO: figure out the true default value
            score_diff: 100,        // TODO: figure out the true default value
            score_diff_double: 100, // TODO: figure out the true default value
            exam1: None,
            exam2: None,
            exam3: None,
        }
    }
}

impl std::fmt::Debug for CourseMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "COURSE:{:?}", self.course)?;
        writeln!(f, "LEVEL:{}", self.level)?;
        if !self.balloon.is_empty() {
            write!(f, "BigBalloon:{}", self.balloon[0])?;
            for count in &self.balloon[1..] {
                write!(f, ",{}", count)?;
            }
            writeln!(f)?;
        }
        if !self.balloon_double.is_empty() {
            write!(f, "BigBalloon (Double):{}", self.balloon_double[0])?;
            for count in &self.balloon_double[1..] {
                write!(f, ",{}", count)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "SCOREDIFF:{}", self.score_diff)?;
        writeln!(f, "SCOREDIFF (Double):{}", self.score_diff_double)?;
        writeln!(f, "SCOREINIT:{}", self.score_init)?;
        write!(f, "SCOREINIT (Double):{}", self.score_init_double)?;
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
