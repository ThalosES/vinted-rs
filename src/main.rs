use vinted_rs::{model::filter::Filter, queries};

#[tokio::main]
async fn main() {
    let mut vinted = queries::VintedWrapper::new();

    vinted.refresh_cookies().await.unwrap();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    vinted.get_item(filter).await.unwrap();
}
