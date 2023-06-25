use rand::Rng;
use regex::bytes::Regex;
// use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;
use regex::Error as RegexError;
use std::str::from_utf8;
use std::str::Utf8Error;
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
    #[error("Regex Init Error")]
    RegexError(#[from] RegexError),
}

struct ValidDomainList;

impl ValidDomainList {
    const DOMAINS: [&'static str; 18] = [
        "fr", "be", "es", "lu", "nl", "lt", "de", "at", "it", "uk", "pt", "com", "cz", "sk", "pl",
        "se", "ro", "hu",
    ];

    fn random_host() -> String {
        let random_index = rand::thread_rng().gen_range(0..Self::DOMAINS.len());
        Self::DOMAINS[random_index].to_string()
    }
}

static RE: OnceCell<Regex> = OnceCell::new();


fn get_regex() -> Result<&'static Regex, RegexError> {
    RE.get_or_try_init(|| (Regex::new(r"cf_bm=([^;]+)")))
}

pub async fn refresh_cookie() -> Result<(String, String), CookieError> {
    let client = reqwest::Client::new();
    let host = ValidDomainList::random_host();
    let request = format!("https://www.vinted.{host}/auth/token_refresh");
    let res = client
        .post(request)
        .header("User-Agent", "PostmanRuntime/7.32.3")
        .send()
        .await?;

    let headers = res.headers();

    let get_all = headers.get_all("set-cookie");

    let Some(cookie_to_parse) = get_all.into_iter().last() else {
        return Err(CookieError::NoLastElement);
    };

    let cook = cookie_to_parse.as_bytes();

    let Some(captures) = get_regex()?.captures(cook) else{
        return Err(CookieError::NoCaptures);
        };
    let Some(cf_bm_value_bytes) = captures.get(1) else {
        return Err(CookieError::NoMatching);
        };

    let cf_bm_value: String = from_utf8(cf_bm_value_bytes.as_bytes())?.to_string();

    Ok((cf_bm_value, host))
}

pub async fn get_item() {}
