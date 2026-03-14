pub mod config;
pub mod db;
pub mod http;

use axum::Router;
use config::AppConfig;
use db::OracleClient;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ramanujan_oracle_tu=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    AppConfig::load_default()
}

pub fn build_app(config: AppConfig) -> Router {
    let oracle_client = OracleClient::new(config.oracle);
    http::router(oracle_client)
}
