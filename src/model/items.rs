use std::fmt;

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


impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            writeln!(f, "{}", item)?;
            writeln!(f, "----------------------")?;
        }

        Ok(())
    }
}