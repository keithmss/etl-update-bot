use crate::api;
use crate::configuration;
use crate::discord::path_container::PathContainer;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::*, prelude::*};

#[command]
// Handle the update command from discord.
pub(super) async fn update(context: &Context, message: &Message) -> CommandResult {
    // `unwrap` Is fine here because `PathContainer` is known to be in the `TypeMap`.
    #[rustfmt::skip]
    let path = context.data.read().await.get::<PathContainer>().unwrap().clone();
    let servers = configuration::get_servers(&path).await?;
    for server in servers {
        let address = server.get_address();
        let token = server.get_token();
        let reply = match api::update(&address, token).await {
            Ok(_) => format!("Updated server: `{address}`."),
            Err(why) => format!("Error updating server: `{address}` because of `{why}`."),
        };
        message.channel_id.say(&context.http, reply).await?;
    }
    Ok(())
}
