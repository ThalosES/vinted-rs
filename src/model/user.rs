use std::fmt;

use crate::model::{Deserialize, Serialize};
#[cfg(feature = "redis")]
use crate::model::{FromRedisValue, ToRedisArgs};

use super::{payment_method::PayInMethod, photo::Photo};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct User {
    id: i64,
    login: String, //usernameredis_macros
    photo: Photo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct AdvancedUser {
    id: i64,
    login: String, //username
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<Photo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_loged_on_ts: Option<String>,
    expose_location: bool,
    country_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    city_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,

    accepted_pay_in_methods: Vec<PayInMethod>,
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
