#[allow(non_camel_case_types)]
pub enum Condition {
    // condition(read, gold)
    g(f64, f64),
    jp(u16, u16),
    jg(u16, u16),
    jb(u16, u16),
    s(u32, u32),
    r(u16, u16),
    h(u16, u16),
    c(u16, u16),
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::g(r, g) => write!(f, "g,{},{}", r, g),
            Self::jp(r, g) => write!(f, "jp,{},{}", r, g),
            Self::jg(r, g) => write!(f, "jg,{},{}", r, g),
            Self::jb(r, g) => write!(f, "jb,{},{}", r, g),
            Self::s(r, g) => write!(f, "s,{},{}", r, g),
            Self::r(r, g) => write!(f, "r,{},{}", r, g),
            Self::h(r, g) => write!(f, "h,{},{}", r, g),
            Self::c(r, g) => write!(f, "c,{},{}", r, g),
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Scope {
    m,
    l,
}

pub struct Exam {
    pub condition: Condition,
    pub scope: Scope,
}

impl Exam {
    pub fn from_str(slice: &str) -> Option<Self> {
        let mut values = slice.split(',');
        use Condition::*;
        match values.next() {
            Some("g") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: g(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: g(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("jp") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: jp(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: jp(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("jg") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: jg(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: jg(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("jb") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: jb(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: jb(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("s") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: s(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: s(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("r") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: r(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: r(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("h") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: h(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: h(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            Some("c") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: c(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: c(read_threshold, gold_threshold),
                                            scope: Scope::l,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        None
    }
}

impl std::fmt::Debug for Exam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?}", self.condition, self.scope)
    }
}
