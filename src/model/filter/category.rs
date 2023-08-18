#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};
#[derive(
    Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize, FromRedisValue, ToRedisArgs,
)]
pub struct Category {
    /// Category id given by Vinted
    pub id: i32,
    /// Category name
    pub title: String,
    /// Category Vinted iso code
    pub code: String,
    /// Category's father id
    pub parent_id: i32,
    /// Category URL in Vinted
    pub url: String,
    /// Category URL in Vinted in English
    pub url_en: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Category {
    fn from(row: Row) -> Self {
        Category::builder()
            .id(row.get("id"))
            .title(row.get("title"))
            .code(row.get("code"))
            .parent_id(row.get("parent_id"))
            .url(row.get("url"))
            .url_en(row.get("url_en"))
            .build()
    }
}
