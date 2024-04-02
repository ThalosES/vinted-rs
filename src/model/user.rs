use crate::model::{Deserialize, Serialize};
#[cfg(feature = "redis")]
use crate::model::{FromRedisValue, ToRedisArgs};
use std::fmt;

use super::{payment_method::PayInMethod, photo::Photo};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct User {
    /// Vinted user ID
    pub id: i64,
    // Username
    pub login: String,
    /// User's profile picture URL
    pub photo: Photo,
}
/// All avalible fields for a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct AdvancedUser {
    /// Vinted user ID
    pub id: i64,
    /// Username
    pub login: String,
    /// User's profile picture URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Photo>,
    /// User's real name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    /// User's email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User's birthday
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Last time the user logged in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_loged_on_ts: Option<String>,
    /// If the user wants to expose his location
    pub expose_location: bool,
    /// See [`Country`](crate::model::filter::country::Country)
    pub country_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_id: Option<i32>,
    /// City name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// Payment methods accepted by the user
    pub accepted_pay_in_methods: Vec<PayInMethod>,
}

impl fmt::Display for AdvancedUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Login (Username): {}", self.login)?;
        writeln!(f, "Photo: {:?}", self.photo)?;
        writeln!(f, "Real Name: {:?}", self.real_name)?;
        writeln!(f, "Email: {:?}", self.email)?;
        writeln!(f, "Birthday: {:?}", self.birthday)?;
        writeln!(f, "Gender: {:?}", self.gender)?;
        writeln!(f, "Last Logged On Timestamp: {:?}", self.last_loged_on_ts)?;
        writeln!(f, "Expose Location: {}", self.expose_location)?;
        writeln!(f, "Country ID: {}", self.country_id)?;
        writeln!(f, "City ID: {:?}", self.city_id)?;
        writeln!(f, "City: {:?}", self.city)?;
        writeln!(f, "Path: {:?}", self.path)?;
        writeln!(f, "About: {:?}", self.about)?;
        // writeln!(f, "Accepted Pay In Methods: {}", self.accepted_pay_in_methods)?;

        Ok(())
    }
}
