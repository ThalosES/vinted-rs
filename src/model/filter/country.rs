#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub local_name: String,
    pub iso_code: String,
    pub flag: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Country {
    fn from(row: Row) -> Self {
        Country::builder()
            .id(row.get("id"))
            .name(row.get("name"))
            .local_name(row.get("local_name"))
            .iso_code(row.get("iso_code"))
            .flag(row.get("flag"))
            .build()
    }
}
