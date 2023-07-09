use crate::db::DbController;
use crate::model::filter::material::Material;
use crate::model::filter::Filter;
use crate::queries::VintedWrapperError;
use crate::VintedWrapper;
use bb8_postgres::tokio_postgres::NoTls;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

fn calculate_color_difference(hex_color1: &str, hex_color2: &str) -> f64 {
    let color1 = hex_to_rgb(hex_color1);
    let color2 = hex_to_rgb(hex_color2);

    let r_diff = color1.0 as f64 - color2.0 as f64;
    let g_diff = color1.1 as f64 - color2.1 as f64;
    let b_diff = color1.2 as f64 - color2.2 as f64;

    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).sqrt()
}

fn hex_to_rgb(hex_color: &str) -> (u8, u8, u8) {
    let hex = hex_color.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r, g, b)
}

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
            //TODO: Try to test all are from the same catalog somehow
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_price() {
    let vinted = VintedWrapper::new();
    let min = 50;
    let max = 100;

    let filter: Filter = Filter::builder().price_from(min).price_to(max).build();

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 10);
            let ok: bool = items.items.iter().all(|item| {
                let price: f32 = item.price.parse().unwrap();
                price <= max as f32 && price >= min as f32
            });

            assert!(ok);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_size() {
    let vinted = VintedWrapper::new();
    let size_id = String::from("1568");
    let size_title = String::from("XS");

    let filter: Filter = Filter::builder().size_ids(size_id).build();

    match vinted.get_items(&filter, 20).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 20);
            let ok: bool = items.items.iter().all(|item| item.size_title == size_title);

            assert!(ok);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_material() {
    let vinted = VintedWrapper::new();
    let id = 49;
    let es = String::from("Seda");
    let fr = String::from("Soie");
    let en = String::from("Silk");
    let _material = Material::builder()
        .id(id)
        .material_es(es)
        .material_en(en)
        .material_fr(fr);

    let filter: Filter = Filter::builder().material_ids(id.to_string()).build();

    match vinted.get_items(&filter, 20).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 20);
            //TODO: Check they are all from the same material
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
#[should_panic(expected = "Color is not brownish")]
#[ignore]
async fn test_get_items_by_color() {
    let vinted = VintedWrapper::new();
    let id = 2; //Brown
    let hex = "#663300"; //Brown

    let filter: Filter = Filter::builder().color_ids(id.to_string()).build();

    match vinted.get_items(&filter, 20).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 20);
            let ok: bool = items.items.iter().all(|item| {
                let dif = calculate_color_difference(hex, &item.photo.dominant_color);
                println!("{}{:?}", &item.photo.dominant_color, dif.to_string());
                dif < 200.0
            });
            assert!(ok, "Color is not brownish")
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}
