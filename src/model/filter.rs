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
///
///
///You can create a filter with multiple constraints.
///
///### Example
///
///```rust
///
/// let filter: Filter = Filter::builder()
///      .catalog_ids(String::from("4,16"))
///      .brand_ids(String::from("14,53"))
///      .build();
///
/// // Women shoes and Women clothes that are only from brands Adidas and Nike.
///
///
///```
///
///
/// `price_from` filter should be always <= `price_to` , otherwise Vinted will not find anything
///
#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    ///The search text to filter items by.
    ///### Example
    ///```rust
    ///let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();
    ///```
    ///
    ///
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    ///
    ///
    ///
    ///
    ///The catalog IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().catalog_ids(String::from("4,16")).build();
    /// // Where 4 and 16 are catalog_ids from Vinted
    /// // 4 is catalog_id for Women clothes
    /// // 16 is catalog_id for Women Shoes
    ///```
    #[builder(default, setter(strip_option))]
    pub catalog_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    ///The color IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().color_ids(String::from("1,5")).build();
    /// // Where 1 and 5 are color_ids from Vinted
    /// // 1 is color_id for Black
    /// // 5 is color_id for Pink
    ///```
    ///
    pub color_ids: Option<String>,
    /// The brand IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().brand_ids(String::from("14,53")).build();
    /// // Where 14 and 53 are brand_ids from Vinted
    /// // 14 is brand_id for Adidas
    /// // 53 is brand_id for Nike
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<String>,
    /// The country IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().country_ids(String::from("7,16")).build();
    /// // Where 7 and 16 are country_ids from Vinted
    /// // 7 is country_id for Spain
    /// // 16 is country_id for France
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub countries_ids: Option<String>,
    /// The size IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().size_ids(String::from("1226,102")).build();
    /// // Where 7 and 16 are country_ids from Vinted
    /// // 1226 is size_id for XXXS / 30 / 2
    /// // 102 is size_id for XXS / 32 / 4
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub size_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    /// The article statuses to filter items by.
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().article_status(vec![ArticleStatus::NewTags ,
    /// ArticleStatus::NewNoTags]).build();
    ///```
    ///
    pub article_status: Option<Vec<ArticleStatus>>,
    /// The sort order for the retrieved items.

    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().sort_by(SortBy::PriceAscendant).build();
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub sort_by: Option<SortBy>,
    /// The minimum price of the article
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().price_from(10u32).build();
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub price_from: Option<u32>,
    /// The max price of the article
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().price_from(20u32).build();
    ///```
    ///
    #[builder(default, setter(strip_option))]
    pub price_to: Option<u32>,
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
