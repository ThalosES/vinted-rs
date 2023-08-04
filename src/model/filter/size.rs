#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

// TODO las tallas y las categorias de tallas están solo en Castellano
/**
Size structs are differenciated by parent categories
XL for Men is not the same as XL for children
*/
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    /// Size id given by Vinted
    pub id: i32,
    /// Size name in Spanish
    pub title_es: String,
    /// Size name in English
    pub title_en: String,
    /// Size name in French
    pub title_fr: String,
    /// Size type in Spanish
    pub size_type_es: String,
    /// Size type in English
    pub size_type_en: String,
    /// Size type in French
    pub size_type_fr: String,
    /// Parent-category's id associated with the size type
    pub category_id: i32,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Size {
    fn from(row: Row) -> Self {
        Size::builder()
            .id(row.get("id"))
            .title_es(row.get("title_es"))
            .title_en(row.get("title_en"))
            .title_fr(row.get("title_fr"))
            .size_type_es(row.get("size_type_es"))
            .size_type_en(row.get("size_type_en"))
            .size_type_fr(row.get("size_type_fr"))
            .category_id(row.get("category_id"))
            .build()
    }
}
