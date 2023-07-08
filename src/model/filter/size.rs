#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Size {
    id: i32,
    id_vinted: i32,
    // TODO las tallas y las categorias de tallas están solo en Castellano
    size: String,
    category: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Size {
    fn from(row: Row) -> Self {
        Size::builder()
            .id(row.get("id"))
            .id_vinted(row.get("id_vinted"))
            .size(row.get("size"))
            .category(row.get("category"))
            .build()
    }
}
