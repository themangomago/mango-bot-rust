use serenity::{client::bridge::gateway::ShardManager, model::prelude::*, prelude::*};
use std::{collections::HashMap, sync::Arc};

use super::Database;

pub struct BotOwners;
impl TypeMapKey for BotOwners {
    type Value = Vec<UserId>;
}

pub struct BotId;
impl TypeMapKey for BotId {
    type Value = UserId;
}

pub struct DatabaseManager;
impl TypeMapKey for DatabaseManager {
    type Value = Arc<Mutex<Database>>;
}
