use std::fmt;

use crate::model::{Deserialize, FromRedisValue, Serialize, ToRedisArgs};

use super::{payment_method::PayInMethod, photo::Photo};

#[derive(Debug, Clone, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct User {
    id: i64,
    login: String, //username
    photo: Photo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdvancedUser {
    id: i64,
    login: String, //username
    photo: Option<Photo>,

    real_name: Option<String>,
    email: Option<String>,
    birthday: Option<String>,
    gender: Option<String>,

    last_loged_on_ts: String,
    expose_location: bool,
    country_id: i32,
    city_id: Option<i32>,
    city: Option<String>,

    path: String,
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
        writeln!(f, "Last Logged On Timestamp: {}", self.last_loged_on_ts)?;
        writeln!(f, "Expose Location: {}", self.expose_location)?;
        writeln!(f, "Country ID: {}", self.country_id)?;
        writeln!(f, "City ID: {:?}", self.city_id)?;
        writeln!(f, "City: {:?}", self.city)?;
        writeln!(f, "Path: {}", self.path)?;
        writeln!(f, "About: {:?}", self.about)?;
        // writeln!(f, "Accepted Pay In Methods: {}", self.accepted_pay_in_methods)?;

        Ok(())
    }
}
