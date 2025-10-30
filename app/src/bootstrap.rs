use crate::{config::AppConfig, db, routes, server};
use dotenvy::dotenv;
use blog;

pub async fn run() -> anyhow::Result<()> {
    // 1) load .env + config
    dotenv().ok();
    let cfg = AppConfig::from_env()?;

    // 2) db pool
    let pool = db::make_pool(&cfg.database_url, cfg.db_max_conns).await?;

    // 3) run blog migrations
    blog::migrate::migrate(&pool).await?;

    // 4) build app router
    let app = routes::build_app_router(pool, &cfg);

    // 5) serve
    server::serve(app, &cfg.bind_addr).await?;
    Ok(())
}
