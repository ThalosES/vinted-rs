
use vinted_rs::queries;

#[tokio::main]
async fn main() {
    let mut vinted = queries::VintedWrapper::new();

    vinted.refresh_cookies().await.unwrap();

    vinted.get_item().await.unwrap();

}
