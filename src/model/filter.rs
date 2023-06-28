use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub catalog_id: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub color_id: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<Vec<i32>>,
}

