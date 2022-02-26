#[macro_use]
extern crate tracing;

mod api;
mod configuration;
mod discord;
mod logger;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    logger::init();

    // Retrieve configuration path.
    let path = configuration::init()?;

    // Spawn the discord task.
    let discord_task = tokio::spawn(async move { discord::run(&path).await });

    // Await completion of the discord task.
    if let Err(why) = discord_task.await? {
        error!("Error running discord bot: '{why}'.");
    }

    // All tasks have completed.
    Ok(())
}
