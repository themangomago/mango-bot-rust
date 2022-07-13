# Mango Bot - A Discord Bot to Monitor Git Repositories

The purpose of this bot it to monitor provided git repositories for new commits and send a notification to an specific Discord channel.
This bot was made to learn Rust using the Serenity framework.

![Mango Bot](./mango_profile.png)

## Commands
- !ping - Pong!
- add_repo - Add a repo to the list of repos to be watched
- del_repo - Delete a repo from the list of repos to be watched
- list_repos - List the repos to be watched


## Config
Create an `.env` file in the root directory of the project. Enter your client token from the Discord developer portal.
Configure your desired prefix for the commands.

```
# Discord Bot Tokens
CLIENT_TOKEN = "xxx"
CLIENT_PREFIX = "!"
```

## Run
Run the bot with `cargo run`.

## Invite Bot to your Channel
Insert your client id and permissions in the url:

https://discord.com/oauth2/authorize?client_id=!!client_id!!&permissions=!!permissions!!&scope=bot+applications.commands