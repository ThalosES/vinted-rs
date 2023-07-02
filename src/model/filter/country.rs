#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
struct Country {
    id: i32,
    local_name: String,
    iso_code: String,
    flag: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Country {
    fn from(row: Row) -> Self {
        Country::builder()
            .id(row.get("id"))
            .local_name(row.get("local_name"))
            .iso_code(row.get("iso_code"))
            .flag(row.get("flag"))
            .build()
    }
}
