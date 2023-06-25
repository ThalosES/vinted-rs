use once_cell::sync::OnceCell;
use rand::Rng;
use reqwest::Client;
use reqwest_cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::str::Utf8Error;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CookieError {
    #[error("ReqwestError")]
    ReqWestError(#[from] reqwest::Error),
    #[error("Utf8Error")]
    Utf8Error(#[from] Utf8Error),
    #[error("NoLastElement")]
    NoLastElement,
    #[error("NoCapturesError")]
    NoCaptures,
    #[error("NoMatchingError")]
    NoMatching,
}

const DOMAINS: [&'static str; 18] = [
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

    pub async fn get_item(&self) -> Result<(), CookieError> {
        //1. Try request
        //2. If fails: get_cookie
        //3. Needs client?
        //4. Should the host be a parameter?
        /*
        https://www.vinted.es/api/v2/catalog/items

         */
        let client = self.get_client();

        let url = format!(
            "https://www.vinted.{}/api/v2/catalog/items",
            self.host.as_ref().unwrap()
        );

        let res = client.get(url).send().await?.text().await?;

        println!("JSON : {res:?}");

        Ok(())
    }
}
