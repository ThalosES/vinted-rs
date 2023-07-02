#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Brand {
    pub id: i32,
    pub title: String,
    pub url: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Brand {
    fn from(row: Row) -> Self {
        Brand::builder()
            .id(row.get("ID"))
            .title(row.get("TITLE"))
            .url(row.get("URL"))
            .build()
    }
}
