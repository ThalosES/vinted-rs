/*use vinted_rs::model::item::Item;
use vinted_rs::model::items::Items;
*/
use vinted_rs::queries;

#[tokio::main]
async fn main() {

    let mut vinted = queries::VintedWrapper::new();

    vinted.refresh_cookies().await.unwrap();

    vinted.get_item().await.unwrap();

    // queries::get_item(&host, &cookie).await.unwrap();

    /*let item = Item {};

    let items = vec![item];

    let _items: Items = Items::new(items);
    */

}
