#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;
use serde:: {Serialize, Deserialize};

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Country {
    /// Country id given by Vinted
    pub id: i32,
    /// Country's name in French
    pub name: String,
    /// Country's name in the local language
    pub local_name: String,
    /// Country ISO code
    pub iso_code: String,
    /// Country's unicode flag emoji
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
