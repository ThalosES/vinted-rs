use vinted_rs::queries;
use regex::bytes::Regex;


#[tokio::main]
async fn main() {

let Ok(regex) = Regex::new(r"cf_bm=([^;]+)") else{
    panic!("")
};
let cookie = queries::refresh_cookie(regex).await.unwrap();

println!("Cookie : {}" , cookie);
}