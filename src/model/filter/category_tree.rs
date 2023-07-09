#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct CategoryTree {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for CategoryTree {
    fn from(row: Row) -> Self {
        CategoryTree::builder()
            .id(row.get("id"))
            .parent_id(row.get("parent_id"))
            .child_id(row.get("child_id"))
            .build()
    }
}
