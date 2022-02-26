use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use serde::Deserialize;

const DEFAULT_PORT: usize = 6972;

/// `Server` configuration containing connection and authentication settings.
#[derive(Debug, Deserialize)]
struct Configuration {
    server: Server,
}

impl Configuration {
    /// Deserialize provided configuration file contents to a `Configuration`.
    async fn from_file(path: &str) -> Result<Self> {
        let raw = tokio::fs::read(path).await?;
        let content = std::str::from_utf8(&raw)?;
        let configuration = toml::from_str::<Configuration>(content)?;
        Ok(configuration)
    }
}

/// Server configuration settings.
#[derive(Debug, Deserialize)]
struct Server {
    address: String,
    port: Option<usize>,
    script: String,
    token: String,
}

impl Server {
    /// Retrieve the addresses of the configured servers.
    fn get_address(&self) -> String {
        let address = &self.address;
        let port = self.port.unwrap_or(DEFAULT_PORT);
        format!("{address}:{port}")
    }
}

/// Retrieve the `Server` address from the live configuration file.
pub(crate) async fn get_server_address(path: &str) -> Result<String> {
    let configuration = Configuration::from_file(path).await?;
    let address = configuration.server.get_address();
    Ok(address)
}

/// Retrieve the `Server` update script path from the live configuration file.
pub(crate) async fn get_script(path: &str) -> Result<String> {
    let configuration = Configuration::from_file(path).await?;
    Ok(configuration.server.script)
}

/// Retrieve the `Server` token from the live configuration file.
pub(crate) async fn get_token(path: &str) -> Result<String> {
    let configuration = Configuration::from_file(path).await?;
    Ok(configuration.server.token)
}

/// Initialize the application and retrieve a path to the configuration file.
#[inline]
pub(super) fn init() -> Result<String> {
    // Configuration path argument.
    let cfg_argument = Arg::new("configuration")
        .short('c')
        .long("cfg")
        .value_name("path")
        .takes_value(true)
        .help("Path to configuration file.")
        .required(true);

    // Build application.
    let path = Command::new("ETLegacy Updater Server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Used to upgrade servers to the latest ETL development snapshot.")
        .arg(cfg_argument)
        .get_matches()
        .value_of("configuration")
        .ok_or_else(|| anyhow!("Missing path to configuration file."))?
        .to_string();

    // Return path to configuration file.
    Ok(path)
}
