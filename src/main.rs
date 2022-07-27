extern crate dotenv;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::json::Value;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::guild::Member;
use serenity::model::Timestamp;
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};
use std::time::SystemTime;

use serenity::prelude::*;

use data::cache::*;
use discord::{commands::*, framework::*, help::*};

pub mod data;
pub mod discord;

struct Config {
    pub client_token: String,
    pub client_prefix: String,
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
                    check_for_git_updates(Arc::clone(&ctx1)).await;
                    tokio::time::sleep(std::time::Duration::from_secs(20)).await;
                }
            });
            self.is_loop_running.store(true, Ordering::Relaxed);
            println!("Loop started!");
        }
    }

    async fn unknown(&self, _ctx: Context, name: String, _raw: Value) {
        println!("Unknown command received: {}", name);
    }

    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!hello" {
    //         // The create message builder allows you to easily create embeds and messages
    //         // using a builder syntax.
    //         // This example will create a message that says "Hello, World!", with an embed that has
    //         // a title, description, an image, three fields, and a footer.
    //         let msg = msg
    //             .channel_id
    //             .send_message(&ctx.http, |m| {
    //                 m.content("Hello, World!")
    //                     .embed(|e| {
    //                         e.title("This is a title")
    //                             .description("This is a description")
    //                             .image("attachment://mango.png")
    //                             .fields(vec![
    //                                 ("This is the first field", "This is a field body", true),
    //                                 ("This is the second field", "Both fields are inline", true),
    //                             ])
    //                             .field(
    //                                 "This is the third field",
    //                                 "This is not an inline field",
    //                                 false,
    //                             )
    //                             .footer(|f| f.text("This is a footer"))
    //                             // Add a timestamp for the current time
    //                             // This also accepts a rfc3339 Timestamp
    //                             .timestamp(Timestamp::now())
    //                     })
    //                     .add_file("./images/mango.png")
    //             })
    //             .await;

    //         if let Err(why) = msg {
    //             println!("Error sending message: {:?}", why);
    //         }
    //     }
    // }
}

use serenity::model::prelude::ChannelId;

async fn check_for_git_updates(ctx: Arc<Context>) {
    let data = ctx.data.read().await;
    let db = data
        .get::<DatabaseManager>()
        .expect("Expected DatabaseManager in TypeMap.")
        .clone();

    let changes: Vec<data::git_database::GitDatabaseEntry> = db.lock().await.check_for_updates();
    for entry in changes {
        let channel = ChannelId(entry.channel_id);

        let string = format!("New commit: {}/commit/{}", entry.url, entry.commit_hash);
        channel.say(&ctx.http, string).await.unwrap();
    }
    // send message to channel
    //let channel_id = ChannelId(823615033710870568);
    // let _ = channel_id.say(&ctx.http, "Hello world!").await;

    //println!("{:?}", db.lock().await.list());
    //println!(".")
}

#[group]
#[commands(ping, add_repo, del_repo, list_repos, uptime)]
struct General;

#[tokio::main]
async fn main() {
    let config = parse_dotenv_file();

    let mut db = data::GitDatabase::new();
    db.load_or_create_db();
    // let last_hash = db.add_new(
    //     "https://github.com/themangomago/mango-bot-rust",
    //     823615033710870568,
    // );
    //println!("{:?}", last_hash);

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
        .configure(|c| {
            c.owners(owners)
                .prefix(config.client_prefix)
                .on_mention(Some(bot_id))
        })
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
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut data = client.data.write().await;
        data.insert::<BotId>(bot_id);
        data.insert::<DatabaseManager>(Arc::new(Mutex::new(db)));
        data.insert::<Time>(timestamp);
    }
    if let Err(why) = client.start_autosharded().await {
        println!("Error starting client: {:?}", why);
    }
}

fn parse_dotenv_file() -> Config {
    dotenv().ok().expect("Error loading .env file");
    let client_token = env::var("CLIENT_TOKEN").unwrap();
    let client_prefix = env::var("CLIENT_PREFIX").unwrap();
    Config {
        client_token,
        client_prefix,
    }
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
