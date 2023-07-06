#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Category {
    id: i32,
    // TODO creo que estos titulos solo estan en ingles
    title: String,
    code: String,
    parent_id: i32,
    url: String,
    url_en: String,
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
