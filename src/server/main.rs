#[macro_use]
extern crate tracing;

mod api;
mod configuration;
mod logger;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    logger::init();

    // Retrieve configuration path.
    let path = configuration::init()?;

    // Spawn the api task.
    let api_task = tokio::spawn(async move { api::run(&path).await });

    // Await completion of the api task.
    if let Err(why) = api_task.await? {
        error!("Error running api server: '{why}'.");
    }

    // All tasks have completed.
    Ok(())
}
