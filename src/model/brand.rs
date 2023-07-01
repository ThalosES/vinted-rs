use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Brand {
    id: i64,
    title: String,
    url: String,
}
