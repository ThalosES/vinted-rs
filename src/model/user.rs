use crate::model::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i64,
    login: String, //username
    photo: Photo,
}
