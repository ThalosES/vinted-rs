use crate::model::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    id: i32,
    title: String,
    price: i64,
    brand: String,
    color: String,
    size: String,
    photo_url: String,
}

