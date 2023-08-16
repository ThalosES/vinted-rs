use std::fmt;

use crate::model::item::Item;
use crate::model::{Deserialize, Serialize};

use super::item::AdvancedItem;
#[derive(Debug, Serialize, Deserialize)]

pub struct Items {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl Items {
    pub fn new(items: Vec<Item>, pagination: Pagination) -> Self {
        Items { items, pagination }
    }
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, item) in self.items.iter().enumerate() {
            writeln!(f, "Item #{}", index + 1)?;
            writeln!(f, "{}", item)?;
            writeln!(f, "----------------------")?;
        }

        writeln!(f, "Timestamp: {}", self.pagination.timestamp)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub current_page: i32,
    pub total_pages: i32,
    pub total_entries: i32,
    pub per_page: i32,
    #[serde(rename = "time")]
    pub timestamp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdvancedItems {
    pub item: AdvancedItem,
}
