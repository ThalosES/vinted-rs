use crate::model::{Deserialize, Serialize};

use super::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub size_title: String,
    pub brand_title: String,
    //discount: String, (null?)
    pub currency: String,
    pub price: String,
    pub photo: Photo, //Sustituable por "full_size_url"
    pub url: String,
    pub is_visible: i32,
    pub promoted: bool,
    pub favourite_count: i32,
}
