pub mod course;
pub mod meta;

pub use course::Course;
pub use meta::Meta;
mod parse;

#[derive(Debug, Default)]
pub struct Chart {
    pub meta: Meta,
    pub easy_course: Option<Course>,
    pub normal_course: Option<Course>,
    pub hard_course: Option<Course>,
    pub oni_course: Option<Course>,
    pub edit_course: Option<Course>,
    pub dan_course: Option<Course>,
    pub tower_course: Option<Course>,
}

impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TJAChart {{")
            .and(writeln!(f, "    TITLE:{}", self.meta.title.get(&[])))
            .and(writeln!(f, "    SUBTITLE:{}", self.meta.subtitle.get(&[])))
            .and(writeln!(
                f,
                "    WAVE:{}",
                self.meta.wave.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(f, "    OFFSET:{}", self.meta.offset))
            .and(writeln!(f, "    DEMOSTART:{}", self.meta.demostart))
            .and(writeln!(
                f,
                "    GENRE:{}",
                self.meta.genre.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(f, "    SCOREMODE:{:?}", self.meta.scoremode))
            .and(writeln!(f, "    LIFE:{}", self.meta.life))
            .and(writeln!(
                f,
                "    BGMOVIE:{}",
                self.meta.bgmovie.as_ref().unwrap_or(&"".to_string())
            ))
            .and(writeln!(
                f,
                "    Easy\tNormal\tHard\tOni\t\tEdit\tDan\t\tTower"
            ))
            .and(writeln!(
                f,
                "    {}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.easy_course.is_some(),
                self.normal_course.is_some(),
                self.hard_course.is_some(),
                self.oni_course.is_some(),
                self.edit_course.is_some(),
                self.dan_course.is_some(),
                self.tower_course.is_some(),
            ))
            .and(writeln!(f, "}}"))
    }
}
