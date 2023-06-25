use vinted_rs::model::item::Item;
use vinted_rs::model::items::Items;
use vinted_rs::queries;

#[tokio::main]
async fn main() {
    let cookie_test = queries::refresh_cookie().await.unwrap();

    println!("Host: {} \nCookie : {}", cookie_test.1, cookie_test.0);

    let item = Item {};

    let items = vec![item];

    let _items: Items = Items::new(items);
}
