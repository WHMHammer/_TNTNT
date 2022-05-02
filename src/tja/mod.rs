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

impl Chart {
    pub fn get(&self, c: course::meta::Course) -> Option<&Course> {
        use course::meta::Course::*;
        match c {
            Easy => self.easy_course.as_ref(),
            Normal => self.normal_course.as_ref(),
            Hard => self.hard_course.as_ref(),
            Oni => self.oni_course.as_ref(),
            Edit => self.edit_course.as_ref(),
            Dan => self.dan_course.as_ref(),
            Tower => self.tower_course.as_ref(),
        }
    }

    pub fn get_mut(&mut self, c: course::meta::Course) -> &mut Course {
        use course::meta::Course::*;
        match c {
            Easy => {
                if self.easy_course.is_none() {
                    self.easy_course = Some(Course::default());
                }
                self.easy_course.as_mut().unwrap()
            }
            Normal => {
                if self.normal_course.is_none() {
                    self.normal_course = Some(Course::default());
                }
                self.normal_course.as_mut().unwrap()
            }
            Hard => {
                if self.hard_course.is_none() {
                    self.hard_course = Some(Course::default());
                }
                self.hard_course.as_mut().unwrap()
            }
            Oni => {
                if self.oni_course.is_none() {
                    self.oni_course = Some(Course::default());
                }
                self.oni_course.as_mut().unwrap()
            }
            Edit => {
                if self.edit_course.is_none() {
                    self.edit_course = Some(Course::default());
                }
                self.edit_course.as_mut().unwrap()
            }
            Dan => {
                if self.dan_course.is_none() {
                    self.dan_course = Some(Course::default());
                }
                self.dan_course.as_mut().unwrap()
            }
            Tower => {
                if self.tower_course.is_none() {
                    self.tower_course = Some(Course::default());
                }
                self.tower_course.as_mut().unwrap()
            }
        }
    }
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
