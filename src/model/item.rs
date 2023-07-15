use super::photo::Photo;
use crate::model::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub size_title: String,
    pub brand_title: String,
    pub currency: String,
    pub price: String,
    pub photo: Photo,
    pub url: String,
    pub is_visible: i32,
    pub promoted: bool,
    pub favourite_count: i32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Size Title: {}", self.size_title)?;
        writeln!(f, "Brand Title: {}", self.brand_title)?;
        writeln!(f, "Currency: {}", self.currency)?;
        writeln!(f, "Price: {}", self.price)?;
        writeln!(f, "Photo: {}", self.photo)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Is Visible: {}", self.is_visible)?;
        writeln!(f, "Promoted: {}", self.promoted)?;
        writeln!(f, "Favourite Count: {}", self.favourite_count)?;

        Ok(())
    }
}
