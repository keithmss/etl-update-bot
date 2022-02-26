use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

/// Thread safe container for `ShardManager` instances.
pub(super) struct ShardContainer;

impl TypeMapKey for ShardContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
