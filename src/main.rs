use std::net::SocketAddr;

use anyhow::Context;
use ramanujan_oracle_tu::{build_app, load_config, tracing_init};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_init();

    let config = load_config().context("failed to load env.toml")?;
    let app = build_app(config.clone());
    let addr = SocketAddr::from((config.server.host, config.server.port));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind HTTP listener on {addr}"))?;

    axum::serve(listener, app)
        .await
        .context("axum server failed")?;

    Ok(())
}
