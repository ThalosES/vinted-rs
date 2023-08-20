#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct Material {
    /// Material id given by Vinted
    pub id: i32,
    /// Material name in Spanish
    pub material_es: String,
    /// Material name in French
    pub material_fr: String,
    /// Material name in English
    pub material_en: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Material {
    fn from(row: Row) -> Self {
        Material::builder()
            .id(row.get("id"))
            .material_es(row.get("material_es"))
            .material_en(row.get("material_en"))
            .material_fr(row.get("material_fr"))
            .build()
    }
}
