use crate::model::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    id: i64,
    title: String,
    url: String
}