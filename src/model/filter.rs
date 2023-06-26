use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub catalog_id: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub color_id: Option<i32>,
    // id 15 es Mango
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<Vec<Brands>>,
}

#[derive(Debug, Clone)]
pub enum Brands {
    FaitMain,
    Pimkie,
    Berenice,
    Promod,
    Gap,
    HM, // H&M
    NewLook,
    Levis,
    AmericanVintage,
    Zara,
    Accessorize,
    Adidas,
    Mango,
    Topshop,
    Esprit,
    ChocolateSchubar,
    Guess,
    Kickers,
    Bata,
    Nike,
    Lacoste,
}

impl Into<i32> for Brands {
    fn into(self) -> i32 {
        match self {
            Self::FaitMain => 2,
            Self::Pimkie => 3,
            Self::Berenice => 4,
            Self::Promod => 5,
            Self::Gap => 6,
            Self::HM => 7,
            Self::NewLook => 9,
            Self::Levis => 10,
            Self::AmericanVintage => 11,
            Self::Zara => 12,
            Self::Accessorize => 13,
            Self::Adidas => 14,
            Self::Mango => 15,
            Self::Topshop => 16,
            Self::Esprit => 17,
            Self::ChocolateSchubar => 19,
            Self::Guess => 20,
            Self::Kickers => 22,
            Self::Bata => 24,
            Self::Nike => 53,
            Self::Lacoste => 304,
        }
    }
}
