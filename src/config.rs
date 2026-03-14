use std::{fs, net::IpAddr, path::Path};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub oracle: OracleConfig,
}

impl AppConfig {
    pub fn load_default() -> Result<Self> {
        Self::load_from_file("env.toml")
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

        toml::from_str(&raw)
            .with_context(|| format!("failed to parse config file {}", path.display()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OracleConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub service_name: String,
}

impl OracleConfig {
    pub fn connect_string(&self) -> String {
        format!("//{}:{}/{}", self.host, self.port, self.service_name)
    }
}
