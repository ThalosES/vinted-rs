#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Color {
    pub id: i32,
    pub title: String,
    pub hex: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Color {
    fn from(row: Row) -> Self {
        Color::builder()
            .id(row.get("id"))
            .title(row.get("title"))
            .hex(row.get("hex"))
            .build()
    }
}
