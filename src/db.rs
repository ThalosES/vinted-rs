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

#[derive(Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    PoolError(#[from] RunError<bb8_postgres::tokio_postgres::Error>),
    #[error(transparent)]
    PgError(#[from] bb8_postgres::tokio_postgres::Error),
}

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
    pub async fn new(uri: &str, pool_size: u32, tls: Tls) -> Result<DbController<Tls>, DbError> {
        let manager = PostgresConnectionManager::new_from_stringlike(uri, tls)?;

        let pool = Pool::builder().max_size(pool_size).build(manager).await?;

        Ok(DbController { pool })
    }

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
