/*!
 [![github]](https://github.com/TuTarea/vinted-rs/)&ensp;[![crates-io]](https://crates.io/crates/vinted-rs)&ensp;[![docs-rs]](crate)

 [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
 [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
 [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Vinted API Wrapper
Welcome to this first release of our Vinted API Wrapper, as a disclaimer, we would like to notify that this is an open-source project to help programmers to take advantage of Vinted **public** API routes.

## API Authentication
Cookie automatic authentication using CookieStore

## API Functionality
- Retrieve serialized items

- Optional filter use

### Current available filtering
- Colors
- Categories
- Brands
- Product status
- Order by

- Search via raw text

# Examples

```rust
use bb8_postgres::tokio_postgres::NoTls;
use lazy_static::lazy_static;
use std::env;
use vinted_rs::{db::DbController, queries::Host, Filter, VintedWrapper};

lazy_static! {
    pub static ref POSTGRES_DB: String =
        std::env::var("POSTGRES_DB").unwrap_or(String::from("vinted-rs"));
    pub static ref POSTGRES_USER: String =
        std::env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    pub static ref POSTGRES_PASSWORD: String =
        std::env::var("POSTGRES_PASSWORD").unwrap_or(String::from("postgres"));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = dotenvy::dotenv(); //Load at runtime

    if args.len() < 2 {
        println!("Please provide the host as a command-line parameter.");
        return;
    }

    let host_arg = args[1].as_str();
    let host: Host = host_arg.into();

    let vinted = VintedWrapper::new_with_host(host);
    println!("Host: {}", vinted.get_host());

    let db_uri = &format!(
        "postgres://{}:{}@localhost/{}?sslmode=disable",
        POSTGRES_USER.clone(),
        POSTGRES_PASSWORD.clone(),
        POSTGRES_DB.clone()
    );

    let db = DbController::new(&db_uri, 5, NoTls)
        .await
        .expect("Broken connection to Database, please set it up correctly");

    let adidas = db.get_brand_by_name(&"Adidas").await.unwrap();
    let nike = db.get_brand_by_name(&"Nike").await.unwrap();

    let brands = format!("{},{}", adidas.id, nike.id);

    let filter = Filter::builder()
        .brand_ids(Some(brands))
        .price_from(Some(15.0))
        .price_to(Some(20.0))
        .build();

    let items = vinted
        .get_items(&filter, 10, None, None, None)
        .await
        .unwrap();

    if items.items.is_empty() {
        println!("No items found");
    } else {
        for item in items.items {
            let advanced = vinted
                .get_advanced_item(item.id, None, None, None)
                .await
                .unwrap();
            println!("{}", advanced);
        }
    }
}


```
*/
#[cfg(feature = "advanced_filters")]
pub mod db;
pub mod model;
pub mod queries;
pub mod utils;
pub use model::filter::Filter;
pub use queries::CookieError;
pub use queries::VintedWrapper;
pub use queries::VintedWrapperError;
#[cfg(test)]
pub mod tests;
