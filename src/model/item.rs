use super::{photo::Photo, user::AdvancedUser};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedItem {
    // Products filter info
    pub id: i64,
    pub title: String,
    pub description: String,
    pub size_title: String,
    pub brand_title: String,
    pub composition: String,
    pub extra_conditions: String,
    pub brand_id: i32,
    pub size_id: i32,
    pub status_id: i32,
    pub disposal_conditions: i32,
    pub catalog_id: i32,
    pub color1_id: i32,
    pub color2_id: Option<i32>,
    pub package_size_id: i32,
    //Location
    pub country_id: i32,
    pub city_id: Option<i32>,
    pub city: Option<String>,

    //Stats
    pub active_bid_count: i32,
    pub favourite_count: i32,
    pub view_count: i32,
    pub moderation_status: i32,
    //pub last_push_up_at: DateTime<Utc>,
    pub related_catalog_ids: Vec<i32>,

    // Pricing
    pub original_price_numeric: i64,
    pub currency: String,
    pub price_numeric: i64,

    // Order by stats
    // pub created_at_ts: Timestamp<Utc>,
    // pub updated_at_ts: Timestamp<Utc>,
    // pub user_updated_at_ts: Timestamp<Utc>,

    // Asets
    pub photos: Vec<Photo>,
    pub url: String,
    pub user: AdvancedUser,

    // Some flags
    pub is_for_sell: bool,
    pub is_for_swap: bool,
    pub is_for_give_away: bool,
    pub is_handicraft: bool,
    pub is_processing: bool,
    pub is_draft: bool,
    pub promoted: bool,
    pub package_size_standard: bool,
    pub related_catalogs_enabled: bool,
    // More flags, just in i32
    pub is_hidden: i32,
    pub is_reserved: i32,
    pub reserved_for_user_id: Option<i32>,
    pub is_visible: i32,
    pub is_unisex: i32,
    pub is_closed: i32,
}

impl fmt::Display for AdvancedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Size Title: {}", self.size_title)?;
        writeln!(f, "Brand Title: {}", self.brand_title)?;
        writeln!(f, "Currency: {}", self.currency)?;
        writeln!(f, "Price: {}", self.price_numeric)?;
        //writeln!(f, "Photo: {}", self.photo)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Is Visible: {}", self.is_visible)?;
        writeln!(f, "Promoted: {}", self.promoted)?;
        writeln!(f, "Favourite Count: {}", self.favourite_count)?;

        Ok(())
    }
}
