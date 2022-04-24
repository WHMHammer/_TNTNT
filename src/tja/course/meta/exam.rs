#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Condition {
    g,
    jp,
    jg,
    jb,
    s,
    r,
    h,
    c,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Scope {
    m,
    l,
}

#[derive(Debug)]
pub struct Exam {
    pub condition: Condition,
    pub red: f64,
    pub gold: f64,
    pub scope: Scope,
}

impl Exam {
    pub fn from_str(_slice: &str) -> Option<Self> {
        None
        // TODO: implement parsing
    }
}
