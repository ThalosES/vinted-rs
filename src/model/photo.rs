use std::fmt;

use crate::model::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct Photo {
    pub id: i64,
    pub url: String,
    pub dominant_color: String,
    pub dominant_color_opaque: String,
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Dominant Color: {}", self.dominant_color)?;
        writeln!(f, "Dominant Color Opaque: {}", self.dominant_color_opaque)?;

        Ok(())
    }
}
