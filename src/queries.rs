use regex::bytes::Regex;
// use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;
use regex::Error as RegexError;
use std::str::from_utf8;
use std::str::Utf8Error;
use thiserror::Error;

/*
"https://www.vinted.fr/auth/token_refresh"
"https://www.vinted.be/auth/token_refresh"
"https://www.vinted.es/auth/token_refresh"
"https://www.vinted.lu/auth/token_refresh"
"https://www.vinted.nl/auth/token_refresh"
"https://www.vinted.lt/auth/token_refresh"
"https://www.vinted.de/auth/token_refresh"
"https://www.vinted.at/auth/token_refresh"
"https://www.vinted.it/auth/token_refresh"
"https://www.vinted.co.uk/auth/token_refresh"
"https://www.vinted.pt/auth/token_refresh"
"https://www.vinted.com/auth/token_refresh"
"https://www.vinted.cz/auth/token_refresh"
"https://www.vinted.sk/auth/token_refresh"
"https://www.vinted.pl/auth/token_refresh"
"https://www.vinted.se/auth/token_refresh"
"https://www.vinted.ro/auth/token_refresh"
"https://www.vinted.hu/auth/token_refresh"
*/

static RE: OnceCell<Regex> = OnceCell::new();

fn get_regex() -> Result<&'static Regex, RegexError> {
    RE.get_or_try_init(|| (Regex::new(r"cf_bm=([^;]+)")))
}

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

pub async fn refresh_cookie() -> Result<String, CookieError> {
    let client = reqwest::Client::new();
    let host = "https://www.vinted.be";
    let request = format!("{host}/auth/token_refresh");
    let res = client
        .post(request)
        // .header("User-Agent", "PostmanRuntime/7.32.3")
        // .header("Host",  host)
        // .header("Connection", "keep-alive")
        // .header("Accept", "*/*")
        // .header("Accept-Encoding", "gzip, deflate, br")
        .send()
        .await?;

    let headers = res.headers();
    /*     .json()
        .await?;
    */
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

    Ok(cf_bm_value)
}

pub async fn get_item() {}
