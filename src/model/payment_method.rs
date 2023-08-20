use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct PayInMethod {
    id: i32,
    code: String,
    requires_credit_card: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_tracking_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    translated_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    method_change_possible: bool,
}

impl fmt::Display for PayInMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Code: {}", self.code)?;
        writeln!(f, "Requires Credit Card: {}", self.requires_credit_card)?;
        writeln!(f, "Event Tracking Code: {:?}", self.event_tracking_code)?;
        writeln!(f, "Icon: {:?}", self.icon)?;
        writeln!(f, "Enabled: {:?}", self.enabled)?;
        writeln!(f, "Translated Name: {:?}", self.translated_name)?;
        writeln!(f, "Note: {:?}", self.note)?;
        writeln!(f, "Method Change Possible: {}", self.method_change_possible)?;

        Ok(())
    }
}
