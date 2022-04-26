#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Condition {
    g(f64, f64),
    jp(u16, u16),
    jg(u16, u16),
    jb(u16, u16),
    s(u32, u32),
    r(u16, u16),
    h(u16, u16),
    c(u16, u16),
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
    pub scope: Scope,
}

impl Exam {
    pub fn from_str(slice: &str) -> Option<Self> {
        let mut values = slice.split(',');
        match values.next() {
            Some("g") => {
                if let Some(red_threshold) = values.next() {
                    if let Ok(read_threshold) = red_threshold.parse() {
                        if let Some(gold_threshold) = values.next() {
                            if let Ok(gold_threshold) = gold_threshold.parse() {
                                match values.next() {
                                    Some("m") => {
                                        return Some(Self {
                                            condition: Condition::g(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::g(read_threshold, gold_threshold),
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
                                            condition: Condition::jp(
                                                read_threshold,
                                                gold_threshold,
                                            ),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::jp(
                                                read_threshold,
                                                gold_threshold,
                                            ),
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
                                            condition: Condition::jg(
                                                read_threshold,
                                                gold_threshold,
                                            ),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::jg(
                                                read_threshold,
                                                gold_threshold,
                                            ),
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
                                            condition: Condition::jb(
                                                read_threshold,
                                                gold_threshold,
                                            ),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::jb(
                                                read_threshold,
                                                gold_threshold,
                                            ),
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
                                            condition: Condition::s(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::s(read_threshold, gold_threshold),
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
                                            condition: Condition::r(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::r(read_threshold, gold_threshold),
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
                                            condition: Condition::h(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::h(read_threshold, gold_threshold),
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
                                            condition: Condition::c(read_threshold, gold_threshold),
                                            scope: Scope::m,
                                        });
                                    }
                                    Some("l") => {
                                        return Some(Self {
                                            condition: Condition::c(read_threshold, gold_threshold),
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
