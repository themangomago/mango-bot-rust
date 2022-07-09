extern crate dotenv;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

use serenity::prelude::*;

use data::cache::*;
use discord::{commands::*, framework::*, help::*};

pub mod data;
pub mod discord;

struct Config {
    pub client_token: String,
}

struct Event {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Event {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    dummy(Arc::clone(&ctx1)).await;
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            });
            self.is_loop_running.store(true, Ordering::Relaxed);
            println!("Loop started!");
        }
    }

    async fn message(&self, _: Context, msg: Message) {}
}

async fn dummy(ctx: Arc<Context>) {
    println!(".")
}

#[group]
#[commands(ping, add_repo, del_repo, list_repos)]
struct General;

#[tokio::main]
async fn main() {
    let config = parse_dotenv_file();

    let mut db = data::Database::new();
    db.add("https://github.com/Elinvynia/bot.git", "0xff");
    println!("{:?}", db.list());

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let http = Http::new(&config.client_token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!").on_mention(Some(bot_id)))
        .on_dispatch_error(dispatch_error)
        .before(before)
        .after(after)
        .help(&HELP)
        .group(&GENERAL_GROUP);

    let mut client = serenity::Client::builder(&config.client_token, intents)
        .event_handler(Event {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<BotId>(bot_id);
        data.insert::<DatabaseManager>(Arc::new(db));
    }
    if let Err(why) = client.start_autosharded().await {
        println!("Error starting client: {:?}", why);
    }
}

fn parse_dotenv_file() -> Config {
    dotenv().ok().expect("Error loading .env file");
    let client_token = env::var("CLIENT_TOKEN").unwrap();
    Config { client_token }
}

///////////////////////////////////////////////////////////////////////////////
// Playground
///////////////////////////////////////////////////////////////////////////////
// // Unsafe Static Mut Variant
// static mut counter_value: u64 = 0;
// static mut counter_started: bool = false;
// static mut channel_id: ChannelId = ChannelId(0);

// async fn playground(ctx: Arc<Context>) {
//     unsafe {
//         if counter_started {
//             counter_value += 1;
//             println!("Counter: {}", counter_value);
//             channel_id
//                 .say(&ctx.http, &format!("Counter: {}", counter_value))
//                 .await
//                 .unwrap();
//         }
//     }
// }

// #[command]
// #[only_in(guilds)]
// async fn count(ctx: &Context, msg: &Message) -> CommandResult {
//     unsafe {
//         counter_started = true;
//         channel_id = msg.channel_id;
//     }

//     Ok(())
// }

// #[command]
// #[only_in(guilds)]
// async fn stop_count(ctx: &Context, msg: &Message) -> CommandResult {
//     unsafe {
//         counter_started = false;
//         channel_id = ChannelId(0);
//     }
//     Ok(())
// }
///////////////////////////////////////////////////////////////////////////////
