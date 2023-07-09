use crate::model::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Photo {
    pub id: i64,
    pub url: String,
    pub dominant_color: String,
    pub dominant_color_opaque: String,
}
