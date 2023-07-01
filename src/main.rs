use bb8_postgres::tokio_postgres::NoTls;
use vinted_rs::{
    db::DbController,
    model::{brand::Brand, filter::Filter},
    queries,
};

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

#[tokio::main]
async fn main() {
    let mut vinted = queries::VintedWrapper::new();

    vinted.refresh_cookies().await.unwrap();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    vinted.get_item(filter).await.unwrap();

    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let b: Brand = db.get_brand_by_name("adidas").await.unwrap();

    println!("DAME ADIDAS OBESO, A VER QUE ME DAS : {b:?}");
}
