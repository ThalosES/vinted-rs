use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Brand {
    pub id: i64,
    pub title: String,
    pub url: String,
}
