#[derive(Clone)]
pub struct Context {
    // TODO: compress flags
    pub measure_index: usize,
    pub scroll: f64,
    pub flag_gogo: bool,
    pub flag_level_hold: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            measure_index: 1,
            scroll: 1.0,
            flag_gogo: false,
            flag_level_hold: false,
        }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(measure {:03}", self.measure_index)?;
        write!(f, ", scroll = {}", self.scroll)?;
        if self.flag_gogo {
            write!(f, ", GOGO")?;
        }
        if self.flag_level_hold {
            write!(f, ", LEVELHOLD")?;
        }
        write!(f, ")")
    }
}
