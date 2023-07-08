use once_cell::sync::OnceCell;
use rand::Rng;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest_cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;
use thiserror::Error;

use crate::model::filter::Filter;
use crate::model::items::Items;

#[derive(Error, Debug)]
pub enum CookieError {
    #[error(transparent)]
    ReqWestError(#[from] reqwest::Error),
    #[error("Error to get cookies")]
    GetCookiesError,
}

#[derive(Error, Debug)]
pub enum VintedWrapperError {
    #[error(transparent)]
    CookiesError(#[from] CookieError),
    #[error("Number of items must be non-zero value")]
    ItemNumberError,
}

impl From<reqwest::Error> for VintedWrapperError{
    fn from(value: reqwest::Error) -> Self {
        VintedWrapperError::CookiesError(CookieError::ReqWestError(value))
    }
}

const DOMAINS: [&str; 18] = [
    "fr", "be", "es", "lu", "nl", "lt", "de", "at", "it", "uk", "pt", "com", "cz", "sk", "pl",
    "se", "ro", "hu",
];

#[derive(Debug, Clone)]
pub enum Host {
    Fr,
    Be,
    Es,
    Lu,
    Nl,
    Lt,
    De,
    At,
    It,
    Uk,
    Pt,
    Com,
    Cz,
    Sk,
    Pl,
    Se,
    Ro,
    Hu,
}

impl From<Host> for &str {
    fn from(val: Host) -> Self {
        match val {
            Host::Fr => DOMAINS[0],
            Host::Be => DOMAINS[1],
            Host::Es => DOMAINS[2],
            Host::Lu => DOMAINS[3],
            Host::Nl => DOMAINS[4],
            Host::Lt => DOMAINS[5],
            Host::De => DOMAINS[6],
            Host::At => DOMAINS[7],
            Host::It => DOMAINS[8],
            Host::Uk => DOMAINS[9],
            Host::Pt => DOMAINS[10],
            Host::Com => DOMAINS[11],
            Host::Cz => DOMAINS[12],
            Host::Sk => DOMAINS[13],
            Host::Pl => DOMAINS[14],
            Host::Se => DOMAINS[15],
            Host::Ro => DOMAINS[16],
            Host::Hu => DOMAINS[17],
        }
    }
}

pub fn random_host<'a>() -> &'a str {
    let random_index = rand::thread_rng().gen_range(0..DOMAINS.len());
    DOMAINS[random_index]
}

static CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct VintedWrapper<'a> {
    host: &'a str,
    cookie_store: Arc<CookieStoreMutex>,
}

impl<'a> Default for VintedWrapper<'a> {
    fn default() -> Self {
        Self::new_with_host(Host::Es)
    }
}

impl<'a> VintedWrapper<'a> {
    pub fn new() -> Self {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        VintedWrapper {
            host: random_host(),
            cookie_store,
        }
    }

    pub fn new_with_host(host: Host) -> Self {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        VintedWrapper {
            host: host.into(),
            cookie_store,
        }
    }

    pub fn set_new_random_host(&mut self) {
        self.host = random_host();
    }

    pub fn set_new_host(&mut self, host: Host) {
        self.host = host.into();
    }

    fn get_client(&self) -> &'static Client {
        CLIENT.get_or_init(|| -> Client {
            reqwest::ClientBuilder::new()
                .cookie_provider(Arc::clone(&self.cookie_store))
                .build()
                .unwrap()
        })
    }

    pub async fn refresh_cookies(&self) -> Result<(), CookieError> {
        self.cookie_store.lock().unwrap().clear();

        let client = self.get_client();

        let request = format!("https://www.vinted.{}/auth/token_refresh", self.host);

        let mut response_cookies = client.post(&request).send().await?;
        let max_retries = 3;
        let mut i = 0;

        while response_cookies.status() != StatusCode::OK && i < max_retries {
            response_cookies = client.post(&request).send().await?;
            i += 1;
            // valorar meter un sleep 0.1 s
        }

        if response_cookies.status() != StatusCode::OK {
            return Err(CookieError::GetCookiesError);
        }

        Ok(())
    }

    fn substitute_if_first(first: &mut bool, url: &mut String) {
        if *first {
            url.replace_range(0..1, "?");
            *first = false;
        }
    }

    /// Retrieves items from the Vinted API based on the provided filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - A reference to a `Filter` struct containing the filter parameters.
    /// * `num` - The number of items to retrieve. Defaults to 1 if not specified.
    ///
    /// # Returns
    ///
    /// A `Result` enum containing either the retrieved `Items` or a `CookieError`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tokio::main;
    /// use crate::vinted_rs::model::items::Items;
    /// use crate::vinted_rs::model::filter::Filter;
    /// use crate::vinted_rs::queries::VintedWrapper;
    /// use crate::vinted_rs::queries::CookieError;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let wrapper = VintedWrapper::new();
    ///     let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();
    ///     let num = 10;
    ///
    ///    match wrapper.get_items(&filter, Some(num)).await {
    ///    Ok(items) => {
    ///        println!("Retrieved {} items: {:?}", items.items.len(), items);
    ///        assert_eq!(items.items.len(), 10);
    ///    }
    ///    Err(err) => match err {
    ///        CookieError::ReqWestError(_) => unreachable!(),
    ///        CookieError::GetCookiesError => (),
    ///    },
    /// }
    /// }
    /// ```
    pub async fn get_items(
        &self,
        filters: &Filter,
        num: u32,
    ) -> Result<Items, VintedWrapperError> {
        if num == 0 {
            return Err(VintedWrapperError::ItemNumberError);
        }

        let domain: &str = &format!("https://www.vinted.{}/api/v2/catalog/items", self.host);

        let cookie_store_clone = self.cookie_store.clone();

        if cookie_store_clone
            .lock()
            .unwrap()
            .get(domain, "/", "__cf_bm")
            .is_none()
        {
            self.refresh_cookies().await?;
        }

        let client = self.get_client();

        let mut first = true;

        let mut url = format!("https://www.vinted.{}/api/v2/catalog/items", self.host);

        // Filtro search text
        if let Some(text) = &filters.search_text {
            url = format!("{url}?search_text={text}");
            first = false;
        }

        // Filtro catalogo
        if let Some(vec) = &filters.catalog_ids {
            let querify_vec: Vec<String> = vec.iter().map(|id| id.to_string()).collect();

            let mut catalog_args: String = format!("&catalog_ids={}", querify_vec.join(","));

            VintedWrapper::substitute_if_first(&mut first, &mut catalog_args);

            url = format!("{url}{catalog_args}");
        }

        // Filtro colores
        if let Some(vec) = &filters.color_ids {
            let querify_vec: Vec<String> = vec.iter().map(|id| id.to_string()).collect();

            let mut color_args: String = format!("&color_ids={}", querify_vec.join(","));

            VintedWrapper::substitute_if_first(&mut first, &mut color_args);

            url = format!("{url}{color_args}");
        }

        if let Some(vec) = &filters.brand_ids {
            let querify_vec: Vec<String> = vec.iter().map(|id| id.to_string()).collect();

            let mut brand_args: String = format!("&brand_ids={}", querify_vec.join(","));

            VintedWrapper::substitute_if_first(&mut first, &mut brand_args);

            url = format!("{url}{brand_args}");
        }

        // TODO terminar de procesar los filtros

        // Limitar el articulo a 1
        url = format!("{url}&per_page={num}");

        let items: Items = client.get(url).send().await?.json().await?;

        Ok(items)
    }
}
