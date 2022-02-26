mod authorization;
mod updater;

use crate::configuration;

use anyhow::Result;
use tonic::transport::Server;

/// Run the API services.
pub(crate) async fn run(path: &str) -> Result<()> {
    // Initialize middleware layers.
    let authorization = authorization::init(path);

    // Initialize services.
    let updater = updater::init(path).await;

    // Parse listening address from configuration.
    let address = configuration::get_server_address(path).await?;

    // Start server.
    info!("Starting on socket: `{address}`.");
    Server::builder()
        .layer(authorization)
        .add_service(updater)
        .serve(address.parse()?)
        .await?;

    // Exit function.
    Ok(())
}
