use vinted_rs::model::item::Item;
use vinted_rs::model::items::Items;
use vinted_rs::queries;

#[tokio::main]
async fn main() {
    let cookie = queries::refresh_cookie().await.unwrap().0;

    println!("Cookie : {}", cookie);

    let item = Item {};

    let items = vec![item];

    let _items: Items = Items::new(items);
}
