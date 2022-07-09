use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::data::cache::*;

#[command]
#[description = "Ping? Pong!"]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let bot_id = data
        .get::<BotId>()
        .expect("Expected CommandCounter in TypeMap.")
        .clone();
    println!("{:?}", bot_id);

    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();
    println!("{:?}", db.list());

    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description = "Add Git repository to watch"]
#[min_args(1)]
#[owners_only]
#[only_in(guilds)]
async fn add_repo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "add_repo").await?;
    Ok(())
}

#[command]
#[description = "Remove git repository to watch"]
#[min_args(1)]
#[owners_only]
#[only_in(guilds)]
async fn del_repo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "del_repo").await?;
    Ok(())
}

#[command]
#[description = "List git repositories from watchlist"]
#[only_in(guilds)]
async fn list_repos(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "list_repos").await?;
    Ok(())
}
