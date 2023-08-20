use std::fmt;

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};

use crate::model::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct Photo {
    pub id: i64,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_color_opaque: Option<String>,
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Dominant Color: {:?}", self.dominant_color)?;
        writeln!(f, "Dominant Color Opaque: {:?}", self.dominant_color_opaque)?;

        Ok(())
    }
}
