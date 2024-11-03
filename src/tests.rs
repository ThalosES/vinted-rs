use lazy_static::lazy_static;

lazy_static! {
    pub static ref POSTGRES_DB: String =
        std::env::var("POSTGRES_DB").unwrap_or(String::from("vinted-rs"));
    pub static ref POSTGRES_USER: String =
        std::env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    pub static ref POSTGRES_PASSWORD: String =
        std::env::var("POSTGRES_PASSWORD").unwrap_or(String::from("postgres"));
    pub static ref DB_URI: String = {
            dotenvy::dotenv().ok(); // Load environment variables from .env file
            format!(
                "postgres://{}:{}@localhost/{}?sslmode=disable",
                *POSTGRES_USER, *POSTGRES_PASSWORD, *POSTGRES_DB
            )
        };
}

#[cfg(test)]
pub mod db;
#[cfg(test)]
pub mod model;
#[cfg(test)]
pub mod queries;
