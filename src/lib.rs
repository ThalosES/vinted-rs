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
    let vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    let items = vinted.get_items(&filter, 5).await.unwrap();

    print!("{items:?}");

    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name: String = String::from("adidas");

    let b: Brand = db.get_brand_by_name(&brand_name).await.unwrap();

    let brands = db.get_brands_by_name(&brand_name).await.unwrap();

    println!("\n\n\n\nBrand {b:?}\n\n\n\n");

    println!("Brands:  {brands:?}");
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
