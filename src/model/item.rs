use crate::model::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    title: String,
    size_title: String,
    brand_title: String,
    //discount: String, (null?)
    currency: String,
    price: String,
    photo: Photo, //Sustituable por "full_size_url"
    url: String,
    is_visible: i32,
    promoted: bool,
    favourite_count: i32,
}


