use once_cell::sync::OnceCell;
use rand::Rng;
use reqwest::Client;
use reqwest_cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;
use thiserror::Error;

use crate::model::items::Items;

#[derive(Error, Debug)]
pub enum CookieError {
    #[error("ReqwestError")]
    ReqWestError(#[from] reqwest::Error),
}

const DOMAINS: [&str; 18] = [
    "fr", "be", "es", "lu", "nl", "lt", "de", "at", "it", "uk", "pt", "com", "cz", "sk", "pl",
    "se", "ro", "hu",
];

fn random_host() -> String {
    let random_index = rand::thread_rng().gen_range(0..DOMAINS.len());
    DOMAINS[random_index].to_string()
}

static CLIENT: OnceCell<Client> = OnceCell::new();

pub struct VintedWrapper {
    host: Option<String>,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Default for VintedWrapper {
    fn default() -> Self {
        Self::new()
    }
}
impl VintedWrapper {
    pub fn new() -> Self {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        VintedWrapper {
            host: None,
            cookie_store,
        }
    }

    fn get_client(&self) -> &'static Client {
        CLIENT.get_or_init(|| -> Client {
            reqwest::ClientBuilder::new()
                .cookie_provider(Arc::clone(&self.cookie_store))
                .build()
                .unwrap()
        })
    }

    pub async fn refresh_cookies(&mut self) -> Result<(), CookieError> {
        self.cookie_store.lock().unwrap().clear();
        let client = self.get_client();
        self.host = Some(random_host()); // Option<String>
        let request = format!(
            "https://www.vinted.{}/auth/token_refresh",
            self.host.as_ref().unwrap()
        );
        client.post(request).send().await?;
        Ok(())
    }

    pub async fn get_item(&mut self) -> Result<(), CookieError> {
        let domain: &str = &format!("vinted.{}", self.host.as_ref().unwrap());

        let cookie_store_clone = self.cookie_store.clone();

        if cookie_store_clone
            .lock()
            .unwrap()
            .get(domain, "/", "__cf_bm")
            .is_none()
        {
            self.refresh_cookies().await?
        }

        let client = self.get_client();

        let url = format!(
            "https://www.vinted.{}/api/v2/catalog/items",
            self.host.as_ref().unwrap()
        );

        let res: Items = client.get(url).send().await?.json().await?;

        println!("JSON :{res:?}");

        Ok(())
    }
}
