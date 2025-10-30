use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn make_pool(db_url: &str, max_conns: u32) -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(db_url)
        .await?;
    Ok(pool)
}
