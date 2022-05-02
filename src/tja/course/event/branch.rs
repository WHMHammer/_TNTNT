#[allow(non_camel_case_types)]
pub enum Thresholds {
    r(u8, u8),
    p(f64, f64),
}

impl std::fmt::Debug for Thresholds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::r(e, m) => {
                write!(f, "r,{},{}", e, m)
            }
            Self::p(e, m) => {
                write!(f, "p,{},{}", e, m)
            }
        }
    }
}

pub struct Branches {
    pub thresholds: Thresholds,
    pub n: Vec<super::Event>,
    pub e: Vec<super::Event>,
    pub m: Vec<super::Event>,
}

impl Branches {
    pub fn from_str(slice: &str) -> Option<Self> {
        let mut values = slice.split(',');
        match values.next() {
            Some("r") => {
                if let Some(e_threshold) = values.next() {
                    if let Ok(e_threshold) = e_threshold.parse() {
                        if let Some(m_threshold) = values.next() {
                            if let Ok(m_threshold) = m_threshold.parse() {
                                return Some(Self {
                                    thresholds: Thresholds::r(e_threshold, m_threshold),
                                    n: Vec::new(),
                                    e: Vec::new(),
                                    m: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
            Some("p") => {
                if let Some(e_threshold) = values.next() {
                    if let Ok(e_threshold) = e_threshold.parse() {
                        if let Some(m_threshold) = values.next() {
                            if let Ok(m_threshold) = m_threshold.parse() {
                                return Some(Self {
                                    thresholds: Thresholds::p(e_threshold, m_threshold),
                                    n: Vec::new(),
                                    e: Vec::new(),
                                    m: Vec::new(),
                                });
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

impl std::fmt::Debug for Branches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#BRANCHSTART {:?}", self.thresholds)?;
        if !self.n.is_empty() {
            writeln!(f, "#N")?;
            for event in &self.n {
                writeln!(f, "{:?}", event)?;
            }
        }
        if !self.e.is_empty() {
            writeln!(f, "#E")?;
            for event in &self.e {
                writeln!(f, "{:?}", event)?;
            }
        }
        if !self.m.is_empty() {
            writeln!(f, "#M")?;
            for event in &self.m {
                writeln!(f, "{:?}", event)?;
            }
        }
        write!(f, "#BRANCHEND")
    }
}
