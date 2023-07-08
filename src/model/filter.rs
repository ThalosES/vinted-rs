use typed_builder::TypedBuilder;

pub mod brand;
pub mod category;
pub mod category_tree;
pub mod colors;
pub mod country;
pub mod material;
pub mod size;

/// Represents a filter for querying items.

/// Trait Implementations:
/// - `TypedBuilder`: Implements the builder pattern for constructing a `Filter` instance.
#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    ///The search text to filter items by.
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    ///The catalog IDs to filter items by. Must be formatted as `(i32,)*` regex.
    #[builder(default, setter(strip_option))]
    pub catalog_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    ///The color IDs to filter items by. Must be formatted as `(i32,)*` regex.
    pub color_ids: Option<String>,
    /// The brand IDs to filter items by. Must be formatted as `(i32,)*` regex.
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<String>,
    /// The country IDs to filter items by. Must be formatted as `(i32,)*` regex.
    #[builder(default, setter(strip_option))]
    pub countries_ids: Option<String>,
    /// The size IDs to filter items by. Must be formatted as `(i32,)*` regex.
    #[builder(default, setter(strip_option))]
    pub size_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    /// The article statuses to filter items by.
    pub article_status: Option<Vec<ArticleStatus>>,
    /// The sort order for the retrieved items.
    #[builder(default, setter(strip_option))]
    pub sort_by: Option<SortBy>,
}

/*
Represents the article status for filtering items.

Variants:
- `NewTags`: The article status for new items with tags.
- `NewNoTags`: The article status for new items without tags.
- `VeryGood`: The article status for items in very good condition.
- `Good`: The article status for items in good condition.
- `Satisfactory`: The article status for items in satisfactory condition.

Trait Implementations:
- `From<&ArticleStatus> for &str>`: Converts an `ArticleStatus` variant to a string slice. */
#[derive(Debug, Clone)]
pub enum ArticleStatus {
    NewTags,
    NewNoTags,
    VeryGood,
    Good,
    Satisfactory,
}

impl From<&ArticleStatus> for &str {
    /// From `&ArticleStatus` to `&str`
    fn from(status: &ArticleStatus) -> Self {
        match *status {
            ArticleStatus::NewTags => "6",
            ArticleStatus::NewNoTags => "1",
            ArticleStatus::VeryGood => "2",
            ArticleStatus::Good => "3",
            ArticleStatus::Satisfactory => "4",
        }
    }
}
/*
Represents the sort order for the retrieved items.

Variants:
- `Relevance`: Sort items by relevance.
- `PriceDescendant`: Sort items by price in descending order.
- `PriceAscendant`: Sort items by price in ascending order.
- `NewestFirst`: Sort items by newest first.

Trait Implementations:
- `From<&SortBy> for &str>`: Converts a `SortBy` variant to a string slice.
*/
#[derive(Debug, Clone)]
pub enum SortBy {
    Relevance,
    PriceDescendant,
    PriceAscendant,
    NewestFirst,
}

impl From<&SortBy> for &str {
    /// From `&SortBy` to `&str`
    fn from(sort: &SortBy) -> Self {
        match *sort {
            SortBy::Relevance => "relevance",
            SortBy::PriceDescendant => "price_high_to_low",
            SortBy::PriceAscendant => "price_low_to_high",
            SortBy::NewestFirst => "newest_first",
        }
    }
}
