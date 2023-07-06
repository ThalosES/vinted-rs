use crate::{db::DbController, model::filter::brand::Brand};
use bb8_postgres::tokio_postgres::NoTls;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

#[tokio::test]
async fn test_get_brand_by_name() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name: String = String::from("adidas");
    //let brand_name : &str = "adidas";

    let b: Brand = db.get_brand_by_name(&brand_name).await.unwrap();

    assert_eq!(
        b,
        Brand::builder()
            .title(String::from("adidas"))
            .id(14)
            .url(String::from("https://www.vinted.es/brand/adidas"))
            .build()
    );
}

#[tokio::test]
async fn test_get_brands_by_name() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name: String = String::from("adidas");
    //let brand_name : &str = "adidas";

    let brands: Vec<Brand> = db.get_brands_by_name(&brand_name).await.unwrap();

    assert_eq!(brands.len(), 38);
}
