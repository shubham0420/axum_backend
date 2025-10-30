// blog/src/migrate.rs
use sqlx::{migrate::Migrator, Pool, Postgres};
use std::path::{Path, PathBuf};

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), sqlx::migrate::MigrateError> {
    // Build an owned PathBuf
    let dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    // NOTE: no &dir here — pass the PathBuf by value
    let m = Migrator::new(dir).await?;
    m.run(pool).await
}
