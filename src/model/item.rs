use crate::model::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    title: String,
    is_visible: bool,
    discount: String,
    currency: String,
    price: String,
    brand: String,
    color: String,
    size: String,
    photo: Photo, //Sustituable por "full_size_url"
    url: String,
    promoted: bool,
    favourite_count: i32,
    size_title: String,
    timestamp: String,
}


