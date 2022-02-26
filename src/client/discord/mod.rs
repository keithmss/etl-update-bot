mod commands;
mod path_container;
mod shard_container;

use crate::configuration;

use anyhow::Result;
use commands::update::*;
use path_container::PathContainer;
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use shard_container::ShardContainer;
use std::collections::HashSet;
use std::sync::Arc;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        let name = ready.user.name;
        info!("Connected as '{name}'.");
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed.");
    }
}

#[group]
#[commands(update)]
struct General;

/// Run the Discord bot.
pub(crate) async fn run(path: &str) -> Result<()> {
    let token = configuration::get_discord_token(path).await?;
    let http = Http::new_with_token(&token);

    // Fetch bot owners.
    let owners = {
        let info = http.get_current_application_info().await?;
        let mut owners = HashSet::new();
        owners.insert(info.owner.id);
        owners
    };

    // Build the framework.
    let framework = StandardFramework::new()
        .configure(|configuration| configuration.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP);

    // Build the client.
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await?;

    // Initialize and pack context data.
    {
        let mut data = client.data.write().await;
        data.insert::<ShardContainer>(client.shard_manager.clone());
        data.insert::<PathContainer>(Arc::new(path.to_string()));
    }

    // Start the shard manager task.
    let shard_manager = client.shard_manager.clone();
    tokio::task::spawn(async move {
        shard_manager.lock().await.shutdown_all().await;
    });

    // Start the discord client.
    client.start().await?;

    Ok(())
}
