/*!
The `db` module provides a database handler for interacting with a PostgreSQL database.

## Dependencies

The `db` module depends on the following external crates:
- `bb8_postgres`: Provides connection pooling for PostgreSQL.
- `tokio_postgres`: Provides the PostgreSQL client.
- `postgres_types`: Provides support for custom types in PostgreSQL.
- `thiserror`: Provides error handling utilities.
 */
use std::fmt::Display;

use bb8_postgres::{
    bb8::{Pool, RunError},
    tokio_postgres::{
        tls::{MakeTlsConnect, TlsConnect},
        Row, Socket,
    },
    PostgresConnectionManager,
};
use postgres_types::ToSql;
use thiserror::Error;

use crate::model::filter::{brand::Brand, category::Category};

const GET_BRAND_BY_NAME: &str = include_str!("sql_queries/GET_BRAND_BY_NAME.sql");
const GET_BRANDS_BY_NAME: &str = include_str!("sql_queries/GET_BRANDS_BY_NAME.sql");
const GET_CATEGORY_BY_NAME: &str = include_str!("sql_queries/GET_CATEGORY_BY_NAME.sql");

/**
Represents an error that can occur during database operations.
Variants:
- `PoolError(RunError<bb8_postgres::tokio_postgres::Error>)`: An error related to the connection pool.
- `PgError(bb8_postgres::tokio_postgres::Error)`: An error related to the PostgreSQL client.
 */
#[derive(Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    PoolError(#[from] RunError<bb8_postgres::tokio_postgres::Error>),
    #[error(transparent)]
    PgError(#[from] bb8_postgres::tokio_postgres::Error),
}

/**
Represents a database controller for interacting with a PostgreSQL database.

Type Parameters:
- `Tls`: The type of the TLS connector used for establishing a secure connection.

Fields:
- `pool`: The connection pool for managing database connections.
 */
pub struct DbController<Tls>
where
    Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pool: Pool<PostgresConnectionManager<Tls>>,
}

impl<Tls> DbController<Tls>
where
    Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    /// Creates a new instance of DbController with the specified database URI, pool size, and TLS connector
    pub async fn new(uri: &str, pool_size: u32, tls: Tls) -> Result<DbController<Tls>, DbError> {
        let manager = PostgresConnectionManager::new_from_stringlike(uri, tls)?;

        let pool = Pool::builder().max_size(pool_size).build(manager).await?;

        Ok(DbController { pool })
    }

    /// Retrieves a brand by its name from the database.
    pub async fn get_brand_by_name<S: AsRef<str> + Sync + ToSql>(
        &self,
        name: &S,
    ) -> Result<Brand, DbError> {
        let conn = self.pool.get().await?;

        let row: Row = conn.query_one(GET_BRAND_BY_NAME, &[name]).await?;

        // Works because From<Row> for Brand is implemented
        let b: Brand = row.into();

        Ok(b)
    }

    /// Retrieves a list of brands matching the provided name pattern from the database.
    pub async fn get_brands_by_name<S: AsRef<str> + Sync + ToSql + Display>(
        &self,
        name: &S,
    ) -> Result<Vec<Brand>, DbError> {
        let conn = self.pool.get().await?;

        let name_to_sql = format!("{}%", &name);

        let rows: Vec<Row> = conn.query(GET_BRANDS_BY_NAME, &[&name_to_sql]).await?;

        // Works because From<Row> for Brand is implemented
        let brands: Vec<Brand> = rows.into_iter().map(|row| row.into()).collect();

        Ok(brands)
    }

    /// Retrieves a category by its title from the database
    pub async fn get_category_by_title<S: AsRef<str> + Sync + ToSql + Display>(
        &self,
        name: &S,
    ) -> Result<Category, DbError> {
        let conn = self.pool.get().await?;
        let row: Row = conn.query_one(GET_CATEGORY_BY_NAME, &[&name]).await?;

        // Works because From<Row> for Category is implemented
        let cat: Category = row.into();

        Ok(cat)
    }
}
