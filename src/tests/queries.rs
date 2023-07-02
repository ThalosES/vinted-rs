use crate::model::filter::Filter;
use crate::{CookieError, VintedWrapper};

#[tokio::test]
async fn test_get_item_query_text() {
    let mut vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    match vinted.get_item(filter).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert_eq!(items.items.len(), 1);
        }
        Err(err) => match err {
            CookieError::ReqWestError(_) => unreachable!(),
            CookieError::GetCookiesError => assert!(true),
        },
    };
}
