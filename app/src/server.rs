use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn serve(app: Router, bind_addr: &str) -> Result<()> {
    let addr: SocketAddr = bind_addr.parse()?;
    println!("Listening on http://{addr}");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
