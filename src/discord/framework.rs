use serenity::{
    framework::standard::{macros::hook, CommandError, DispatchError},
    model::prelude::*,
    prelude::*,
};

#[hook]
pub async fn dispatch_error(
    ctx: &Context,
    msg: &Message,
    error: DispatchError,
    _command_name: &str,
) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(&ctx, format!("Not enough arguments! ({}/{})", given, min))
                .await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let _ = msg
                .channel_id
                .say(&ctx, format!("Too many arguments! ({}/{})", given, max))
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(&ctx, "This command is only for bot owners.")
                .await;
        }
        _ => {
            let _ = msg
                .channel_id
                .say(&ctx, "An error occurred while running this command.")
                .await;
        }
    }
}

#[hook]
// Specify the function to be called prior to every command’s execution. If that function returns true, the command will be executed.
pub async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("{} called {}", msg.author.name, command_name);
    if msg.content.starts_with("!") {
        return true;
    } else {
        return false;
    }
}

#[hook]
// Specify the function to be called after every command’s execution.
pub async fn after(
    _ctx: &Context,
    _msg: &Message,
    command_name: &str,
    error: Result<(), CommandError>,
) {
    if let Err(why) = error {
        println!("Error in {}: {:?}", command_name, why);
    }
}
