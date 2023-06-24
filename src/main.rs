use vinted_rs::queries;
use regex::bytes::Regex;
use vinted_rs::model::item::Item;
use vinted_rs::model::items::Items;


#[tokio::main]
async fn main() {

let Ok(regex) = Regex::new(r"cf_bm=([^;]+)") else{
    panic!("")
};
let cookie = queries::refresh_cookie(regex).await.unwrap();

println!("Cookie : {}" , cookie);

let item = Item {};

let items = vec![item];

let items : Items = Items::new(items);
}