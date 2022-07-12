mod lane;
mod notes;
pub use lane::*;
pub use notes::*;
use std::fmt::Debug;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct CenterPositionConfigurations {
    pub ox: Option<i64>, // "o" means center
    pub oy: Option<i64>, // "o" means center
}

impl Debug for CenterPositionConfigurations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Center(")?;
        if let Some(x) = self.ox {
            write!(f, "{}", x)?;
        } else {
            write!(f, "None")?;
        }
        write!(f, ", ")?;
        if let Some(y) = self.oy {
            write!(f, "{}", y)?;
        } else {
            write!(f, "None")?;
        }
        write!(f, ")")
    }
}
