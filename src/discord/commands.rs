use std::time::SystemTime;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::data::cache::*;

#[command]
#[description = "Ping? Pong!"]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    //println!("{}", msg.channel_id);

    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description = "How long has the bot been online?"]
#[only_in(guilds)]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    //println!("{}", msg.channel_id);
    let data = ctx.data.read().await;
    let time_start = data.get::<Time>().unwrap();
    let time_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Uptime in seconds
    let uptime = time_now - time_start;

    // Uptime in human readable format
    let uptime_str = format!(
        "Uptime: {} days, {} hours, {} minutes, {} seconds",
        uptime / 86400,
        (uptime % 86400) / 3600,
        (uptime % 3600) / 60,
        uptime % 60
    );

    msg.channel_id.say(&ctx.http, uptime_str).await?;

    Ok(())
}

#[command]
#[description = "Add Git repository to watch"]
#[min_args(1)]
#[owners_only]
#[only_in(guilds)]
async fn add_repo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let repo = args.parse::<String>().unwrap();
    let string = format!("Added repo <{}>", repo);

    let data = ctx.data.read().await;
    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();

    let hash = match db.lock().await.add_new(&repo, msg.channel_id.0) {
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
    let repo = args.parse::<String>().unwrap();
    let data = ctx.data.read().await;
    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();
    match db.lock().await.remove(&repo) {
        Ok(_) => msg.channel_id.say(&ctx.http, "Repository removed.").await?,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Error: Couldn't find repository.")
                .await?
        }
    };
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
    let repos = db.lock().await.list(msg.channel_id.0);

    // unroll repos and store in string
    let mut string = String::new();
    string.push_str("\nWatch List:\n");
    if repos.len() == 0 {
        string.push_str("No repository has been added.");
    }
    for repo in repos {
        string.push_str(&format!("{}\n", repo));
    }

    msg.channel_id.say(&ctx.http, string).await?;
    Ok(())
}
