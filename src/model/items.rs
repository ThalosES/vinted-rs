use crate::model::item::Item;
use crate::model::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn new(items: Vec<Item>) -> Self {
        Items { items }
    }
}
