#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct Material {
    id: i32,
    material_es: String,
    material_fr: String,
    material_en: String,
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
