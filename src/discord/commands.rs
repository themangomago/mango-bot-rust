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

    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();
    println!("{:?}", db.lock().await.list());

    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description = "Add Git repository to watch"]
#[min_args(1)]
#[owners_only]
#[only_in(guilds)]
async fn add_repo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let repo = args.parse::<String>().unwrap();
    let string = format!("Added repo {}", repo);

    let data = ctx.data.read().await;
    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();

    let hash = match db.lock().await.add_new(&repo) {
        Ok(hash) => msg.channel_id.say(&ctx.http, string).await?,
        Err(e) => {
            msg.channel_id
                .say(&ctx.http, "Error: Couldn't add repository.")
                .await?
        }
    };

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
    let data = ctx.data.read().await;

    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();
    let repos = db.lock().await.list();

    // unroll repos and store in string
    let mut string = String::new();
    string.push_str("Repositories:\n");
    for repo in repos {
        string.push_str(&format!("{}\n", repo));
    }

    msg.channel_id.say(&ctx.http, string).await?;
    Ok(())
}
