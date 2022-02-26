mod authorization;
mod updater;

use anyhow::Result;

/// Send an update request to the given server.
pub(crate) async fn update(address: &str, token: &str) -> Result<()> {
    updater::run(address, token).await
}
