use serenity::prelude::TypeMapKey;
use std::sync::Arc;

/// Thread safe container for `Path` instances.
pub(super) struct PathContainer;

impl TypeMapKey for PathContainer {
    type Value = Arc<String>;
}
