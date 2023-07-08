use crate::db::DbController;
use crate::model::filter::Filter;
use crate::queries::VintedWrapperError;
use crate::VintedWrapper;
use bb8_postgres::tokio_postgres::NoTls;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

#[tokio::test]
async fn test_get_item_query_text() {
    let vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    match vinted.get_items(&filter, 1).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert_eq!(items.items.len(), 1);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_item_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder().brand_ids(brand.id.to_string()).build();

    match vinted.get_items(&filter, 1).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert_eq!(items.items.get(0).unwrap().brand_title, brand.title);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder().brand_ids(brand.id.to_string()).build();

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            for item in items.items {
                assert_eq!(item.brand_title, brand.title);
            }
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_catalogs_no_db() {
    let vinted = VintedWrapper::new();
    //Woman elements
    let filter: Filter = Filter::builder().catalog_ids(String::from("1904")).build();
    let _substrings = vec![
        "women", "mujer", "femme", "kobiety", "donna", "moterims", "noi", "dames", "zeny", "damen",
        "femei", "mulher",
    ];

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 10);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}
