mod bootstrap;
mod config;
mod db;
mod routes;
mod server;
mod middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}
