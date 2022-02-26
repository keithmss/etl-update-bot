use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use serde::Deserialize;

const DEFAULT_PORT: usize = 6972;

/// Client configuration containing server targets and the discord bot token.
#[derive(Debug, Deserialize)]
struct Configuration {
    discord: Discord,
    server: Option<Vec<Server>>,
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

/// Discord configuration settings.
#[derive(Debug, Deserialize)]
struct Discord {
    token: String,
}

/// Server configuration settings.
#[derive(Debug, Deserialize)]
pub(crate) struct Server {
    address: String,
    port: Option<usize>,
    token: String,
}

impl Server {
    /// Retrieve the address of the `Server`.
    pub(crate) fn get_address(&self) -> String {
        let address = &self.address;
        let port = self.port.unwrap_or(DEFAULT_PORT);
        format!("http://{address}:{port}")
    }

    // Retrieve the token of the `Server`.
    pub(crate) fn get_token(&self) -> &str {
        &self.token
    }
}

/// Retrieve the discord token from the live configuration file.
pub(crate) async fn get_discord_token(path: &str) -> Result<String> {
    let configuration = Configuration::from_file(path).await?;
    let token = configuration.discord.token;
    Ok(token)
}

/// Retrieve a vector of defined `Server`.
pub(crate) async fn get_servers(path: &str) -> Result<Vec<Server>> {
    let configuration = Configuration::from_file(path).await?;
    Ok(configuration.server.unwrap_or_default())
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
