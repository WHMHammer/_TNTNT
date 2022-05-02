pub mod course;
pub mod meta;

pub use course::Course;
pub use meta::Meta;
mod parse;

#[derive(Default)]
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

impl std::fmt::Debug for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.meta)?;
        if let Some(course) = &self.easy_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.normal_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.hard_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.oni_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.edit_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.dan_course {
            write!(f, "\n\n{:?}", course)?;
        }
        if let Some(course) = &self.tower_course {
            write!(f, "\n\n{:?}", course)?;
        }
        Ok(())
    }
}
