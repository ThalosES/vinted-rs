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
use vinted_rs::{
    db::DbController,
    model::{filter::brand::Brand, filter::Filter},
};

use vinted_rs::VintedWrapper;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;



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
        .brand_ids(brands)
        .price_from(15)
        .price_to(20)
        .build();

    let vinted = VintedWrapper::new_with_host(host);

    println!("Host: {}", vinted.get_host());

    let items = vinted.get_items(&filter, 10).await.unwrap();

    if items.items.is_empty() {

        println!("No items found");
    }
    println!("{}", items);

}

```
*/
#[cfg(feature = "advanced_filters")]
pub mod db;
pub mod model;
pub mod queries;
pub use model::filter::Filter;
pub use queries::CookieError;
pub use queries::VintedWrapper;
pub use queries::VintedWrapperError;
#[cfg(test)]
pub mod tests;
