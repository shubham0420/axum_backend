use anyhow::{bail, Context, Result};
use std::{env, time::Duration};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: String,
    pub request_timeout: Duration,
    pub rate_limit_per_window: u64,
    pub rate_limit_window_secs: u64,
    pub db_max_conns: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set in .env")?;
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());

        let request_timeout = env::var("REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(10));

        let rate_limit_per_window = env::var("RATE_LIMIT_PER_WINDOW")
            .ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(100);

        let rate_limit_window_secs = env::var("RATE_LIMIT_WINDOW_SECS")
            .ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(60);

        let db_max_conns = env::var("DB_MAX_CONNS")
            .ok().and_then(|s| s.parse::<u32>().ok()).unwrap_or(10);

        if rate_limit_per_window == 0 {
            bail!("RATE_LIMIT_PER_WINDOW cannot be 0");
        }

        Ok(Self {
            database_url,
            bind_addr,
            request_timeout,
            rate_limit_per_window,
            rate_limit_window_secs,
            db_max_conns,
        })
    }
}
