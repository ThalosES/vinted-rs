use bb8_postgres::tokio_postgres::NoTls;
use vinted_rs::{db::DbController, Filter, VintedWrapper , queries::Host};

#[tokio::main]
async fn main() {
    let db = DbController::new("postgres://postgres:postgres@localhost/vinted-rs", 5, NoTls)
        .await
        .unwrap();

    let adidas = db.get_brand_by_name(&"Adidas").await.unwrap();
    let nike = db.get_brand_by_name(&"Nike").await.unwrap();

    let brands = format!("{},{}", adidas.id, nike.id);

    let filter = Filter::builder()
        .brand_ids(brands)
        .price_from(15)
        .price_to(20)
        .build();

    let vinted = VintedWrapper::new_with_host(Host::Uk);

    let items = vinted.get_items(&filter, 10).await.unwrap();

    println!("{}", items);
}