use crate::model::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i64,
    login: String, //username
    photo: Photo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedUser {
    id: i64,
    login: String, //username
    photo: Photo,

    real_name: Option<String>,
    email: Option<String>,
    birthday: Option<String>,
    gender: Option<String>,

    // last_loged_on_ts: TimeStamp,
    expose_location: bool,
    country_id: i32,
    city_id: Option<i32>,
    city: Option<String>,

    path: String,
    about: Option<String>,
}
