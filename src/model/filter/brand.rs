#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};

#[derive(
    Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize, FromRedisValue, ToRedisArgs,
)]
pub struct Brand {
    /// Brand id given by Vinted
    pub id: i32,
    /// Brand name
    pub title: String,
    /// Brand URl in vinted
    pub url: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Brand {
    fn from(row: Row) -> Self {
        Brand::builder()
            .id(row.get("id"))
            .title(row.get("title"))
            .url(row.get("url"))
            .build()
    }
}
