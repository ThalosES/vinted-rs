use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct PayInMethod {
    id: i32,
    code: String,
    requires_credit_card: bool,
    event_tracking_code: String,
    icon: String,
    enabled: bool,
    translated_name: String,
    note: String,
    method_change_possible: bool,
}

impl fmt::Display for PayInMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Code: {}", self.code)?;
        writeln!(f, "Requires Credit Card: {}", self.requires_credit_card)?;
        writeln!(f, "Event Tracking Code: {}", self.event_tracking_code)?;
        writeln!(f, "Icon: {}", self.icon)?;
        writeln!(f, "Enabled: {}", self.enabled)?;
        writeln!(f, "Translated Name: {}", self.translated_name)?;
        writeln!(f, "Note: {}", self.note)?;
        writeln!(f, "Method Change Possible: {}", self.method_change_possible)?;

        Ok(())
    }
}
