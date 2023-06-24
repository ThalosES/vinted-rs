use crate::model::item::Item;
use crate::model::{Serialize , Deserialize};
#[derive(Debug , Serialize , Deserialize)]
pub struct Items{
    items :Vec<Item>
}

impl Items {
    pub fn new(items : Vec <Item>)-> Self{
        Items { items}
    }
}