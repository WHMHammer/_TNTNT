#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Thresholds {
    r(u8, u8),
    p(f64, f64),
}

#[derive(Debug)]
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
