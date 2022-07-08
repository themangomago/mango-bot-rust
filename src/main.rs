extern crate dotenv;

use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::discord::{commands::*, framework::*, help::*};

use dotenv::dotenv;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::GuildId;
use serenity::prelude::*;

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
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            });
            self.is_loop_running.store(true, Ordering::Relaxed);
            println!("Loop started!");
        }
    }

    async fn message(&self, _: Context, msg: Message) {}

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // let ctx = Arc::new(ctx);
        // if !self.is_loop_running.load(Ordering::Relaxed) {
        //     let ctx1 = Arc::clone(&ctx);
        //     tokio::spawn(async move {
        //         loop {
        //             dummy(Arc::clone(&ctx1)).await;
        //             tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        //         }
        //     });
        //     self.is_loop_running.store(true, Ordering::Relaxed);
        //     println!("Loop started!");
        // }
    }
}

async fn dummy(ctx: Arc<Context>) {
    println!("Alive!")
}

#[group]
#[commands(ping, add_repo)]
struct General;

#[tokio::main]
async fn main() {
    let config = parse_dotenv_file();
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

    if let Err(why) = client.start_autosharded().await {
        println!("Error starting client: {:?}", why);
    }
}

fn parse_dotenv_file() -> Config {
    dotenv().ok().expect("Error loading .env file");
    let client_token = env::var("CLIENT_TOKEN").unwrap();
    Config { client_token }
}
