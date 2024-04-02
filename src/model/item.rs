use super::{photo::Photo, user::AdvancedUser};
use crate::model::{Deserialize, Serialize};
#[cfg(feature = "redis")]
use crate::model::{FromRedisValue, ToRedisArgs};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]

pub struct Item {
    pub id: i64,
    pub title: String,
    pub size_title: String,
    pub brand_title: String,
    pub currency: String,
    pub price: String,
    pub photo: Option<Photo>,
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
        if let Some(ph) = &self.photo {
            writeln!(f, "Photo: {}", ph)?;
        }
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Is Visible: {}", self.is_visible)?;
        writeln!(f, "Promoted: {}", self.promoted)?;
        writeln!(f, "Favourite Count: {}", self.favourite_count)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct AdvancedItem {
    /// Vinted item ID
    pub id: i64,
    /// Item title
    pub title: String,
    /// Item description
    pub description: String,
    /// See [`Size`](crate::model::filter::size::Size)
    #[serde(rename = "size")]
    pub size_title: String,
    /// See [`Brand`](crate::model::filter::brand::Brand)
    #[serde(rename = "brand")]
    pub brand_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_conditions: Option<String>,
    /// See [`Brand`](crate::model::filter::brand::Brand)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_id: Option<i32>,
    /// See [`ArticleStatus`](crate::model::filter::ArticleStatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
    /// Status of the item in French 🇫🇷
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_fr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposal_conditions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color1_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color2_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color2: Option<String>,
    pub package_size_id: i32,
    /// See [`Country`](crate::model::filter::country::Country)
    pub country_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    //Stats
    /// Number of bids on the item
    pub active_bid_count: i32,
    /// Number of times the item was added to favourites
    pub favourite_count: i32,
    /// Number of times the item was viewed
    pub view_count: i32,
    /// Moderation status of the item (Vinted internal use only)
    pub moderation_status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_push_up_at: Option<String>,
    /// Related ['Brand'](crate::model::filter::brand::Brand) IDs
    pub related_catalog_ids: Vec<i32>,

    // Pricing
    /// Original price of the item
    pub original_price_numeric: String,
    /// Currency the item was posted with. See [`Currency`](crate::model::filter::Currency) for valid currencies on Vinted
    pub currency: String,
    /// Current valid price of the item
    pub price_numeric: String,

    // Order by stats
    pub created_at_ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_updated_at_ts: Option<String>,

    // Assets
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_for_user_id: Option<i32>,
    pub is_visible: i32,
    pub is_unisex: i32,
    pub is_closed: i32,
}

impl fmt::Display for AdvancedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(
            f,
            "Size [Title: {}, ID: {:?}]",
            self.size_title, self.size_id
        )?;
        writeln!(
            f,
            "Brand [ Title: {}, ID: {:?}]",
            self.brand_title, self.brand_id
        )?;
        writeln!(f, "Composition: {:?}", self.composition)?;
        writeln!(f, "Extra Conditions: {:?}", self.extra_conditions)?;
        writeln!(f, "Status ID: {:?}", self.status_id)?;
        writeln!(f, "Disposal Conditions: {:?}", self.disposal_conditions)?;
        writeln!(f, "Catalog ID: {:?}", self.catalog_id)?;
        writeln!(f, "Color1 ID: {:?}", self.color1_id)?;
        writeln!(f, "Color2 ID: {:?}", self.color2_id)?;
        writeln!(f, "Package Size ID: {}", self.package_size_id)?;
        writeln!(f, "Country ID: {}", self.country_id)?;
        writeln!(f, "City ID: {:?}", self.city_id)?;
        writeln!(f, "City: {:?}", self.city)?;
        writeln!(f, "Active Bid Count: {}", self.active_bid_count)?;
        writeln!(f, "Favourite Count: {}", self.favourite_count)?;
        writeln!(f, "View Count: {}", self.view_count)?;
        writeln!(f, "Moderation Status: {}", self.moderation_status)?;
        writeln!(f, "Related Catalog IDs: {:?}", self.related_catalog_ids)?;
        writeln!(
            f,
            "Original Price: {} {}",
            self.original_price_numeric, self.currency
        )?;
        writeln!(f, "Price: {} {}", self.price_numeric, self.currency)?;

        writeln!(f, "\nFlags: {{")?;
        writeln!(f, "  is_for_sell: {}", self.is_for_sell)?;
        writeln!(f, "  is_for_swap: {}", self.is_for_swap)?;
        writeln!(f, "  is_for_give_away: {}", self.is_for_give_away)?;
        writeln!(f, "  is_handicraft: {}", self.is_handicraft)?;
        writeln!(f, "  is_processing: {}", self.is_processing)?;
        writeln!(f, "  is_draft: {}", self.is_draft)?;
        writeln!(f, "  promoted: {}", self.promoted)?;
        writeln!(f, "  package_size_standard: {}", self.package_size_standard)?;
        writeln!(
            f,
            "  related_catalogs_enabled: {}",
            self.related_catalogs_enabled
        )?;
        writeln!(f, "  is_hidden: {}", self.is_hidden)?;
        writeln!(f, "  is_reserved: {}", self.is_reserved)?;
        writeln!(f, "  is_visible: {}", self.is_visible)?;
        writeln!(f, "  is_unisex: {}", self.is_unisex)?;
        writeln!(f, "  is_closed: {}", self.is_closed)?;
        writeln!(f, "}}\n")?;

        for (num, photo) in self.photos.iter().enumerate() {
            writeln!(f, "Pic {})", num)?;
            writeln!(f, "{}", photo)?;
        }

        writeln!(f, "URL: {}\n", self.url)?;

        writeln!(f, "@@@@@@@@@@@@@@@@@@@@@@@@@")?;
        writeln!(f, "{}", self.user)?;
        writeln!(f, "@@@@@@@@@@@@@@@@@@@@@@@@@")?;

        Ok(())
    }
}
