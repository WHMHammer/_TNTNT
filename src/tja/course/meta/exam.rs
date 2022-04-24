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
    pub fn from_str(slice: &str) -> Option<Self> {
        use Condition::*;
        use Scope::*;
        let mut values = slice.split(',');
        let condition = match values.next() {
            Some("g") => g,
            Some("jp") => jp,
            Some("jg") => jg,
            Some("jb") => jb,
            Some("s") => s,
            Some("r") => r,
            Some("h") => h,
            Some("c") => c,
            _ => {
                return None;
            }
        };
        let red = if let Some(value) = values.next() {
            if let Ok(value) = value.parse() {
                value
            } else {
                return None;
            }
        } else {
            return None;
        };
        let gold = if let Some(value) = values.next() {
            if let Ok(value) = value.parse() {
                value
            } else {
                return None;
            }
        } else {
            return None;
        };
        let scope = match values.next() {
            Some("m") => m,
            Some("l") => l,
            _ => {
                return None;
            }
        };
        Some(Self {
            condition,
            red,
            gold,
            scope,
        })
    }
}
