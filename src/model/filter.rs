use typed_builder::TypedBuilder;

pub mod brand;
pub mod category;
pub mod category_tree;
pub mod colors;
pub mod country;
pub mod material;
pub mod size;

// TODO valorar si poner que lo que está dentro del Option sea & referencia inmutable
// TODO en vez de que consuma valor
// TODO valorar si pasar un id con i32 o el id ya con el String Vec<i32> o Vec<String>

// TODO valorar si es mejor tener las categorias cargadas en memoria directamente o en la BBDD ya que se podrían cargar "facilmente" con el json de las categorias
#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub catalog_ids: Option<Vec<i32>>,
    #[builder(default, setter(strip_option))]
    pub color_ids: Option<Vec<i32>>,
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<Vec<i32>>,
    #[builder(default, setter(strip_option))]
    pub countries_ids: Option<Vec<i32>>,
    #[builder(default, setter(strip_option))]
    pub size_ids: Option<Vec<i32>>,
    #[builder(default, setter(strip_option))]
    pub article_status : Option<Vec<ArticleStatus>>,
    #[builder(default, setter(strip_option))]
    pub sort_by : Option<SortBy>
}

#[derive(Debug, Clone)]
pub enum ArticleStatus {
    NewTags,
    NewNoTags,
    VeryGood,
    Good,
    Satisfactory,
}

/// Para pasar de `ArticleStatus` a `i32` que es el id de vinted
impl From<ArticleStatus> for i32 {
    fn from(status: ArticleStatus) -> Self {
        match status {
            ArticleStatus::NewTags => 6,
            ArticleStatus::NewNoTags => 1,
            ArticleStatus::VeryGood => 2,
            ArticleStatus::Good => 3,
            ArticleStatus::Satisfactory => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SortBy {
    Relevance,
    PriceDescendant,
    PriceAscendant,
    NewestFirst,
}

impl From<SortBy> for &str {
    fn from(sort: SortBy) -> Self {
        match sort {
            SortBy::Relevance => "relevance",
            SortBy::PriceDescendant => "price_high_to_low",
            SortBy::PriceAscendant => "price_low_to_high",
            SortBy::NewestFirst => "newest_first",
        }
    }
}
