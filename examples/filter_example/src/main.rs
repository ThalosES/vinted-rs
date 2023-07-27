use bb8_postgres::tokio_postgres::NoTls;

use std::env;
use vinted_rs::{db::DbController, queries::Host, Filter, VintedWrapper};


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide the host as a command-line parameter.");
        return;
    }

    let host_arg = args[1].as_str();
    let host: Host = host_arg.into();

    let db = DbController::new("postgres://postgres:postgres@localhost/vinted-rs", 5, NoTls)
        .await
        .unwrap();

    let adidas = db.get_brand_by_name(&"Adidas").await.unwrap();
    let nike = db.get_brand_by_name(&"Nike").await.unwrap();

    let brands = format!("{},{}", adidas.id, nike.id);

    let filter = Filter::builder()
        .brand_ids(Some(brands))
        .price_from(Some(15))
        .price_to(Some(20))
        .build();

    let vinted = VintedWrapper::new_with_host(host);

    println!("Host: {}", vinted.get_host());

    let items = vinted.get_items(&filter, 10).await.unwrap();

    if items.items.is_empty() {

        println!("No items found");
    }
    println!("{}", items);
}
