# OOT-Discord-Bot

A discord bot that can analyze the Ocarina of Time randomizer spoiler log.

## How to use
1. Just upload the spoiler log json file to a Discord channel where the bot has read and write access.
![Example](/docs/example.png)

## Dev

### How to run locally
1. Run `cargo shuttle run`

### How to install to Discord
1. Make a Discord bot like so: https://www.shuttle.rs/blog/2022/09/14/serentity-discord-bot
2. Deploy on shuttle


### How to deploy
1. Create Secrets.toml like so:
```toml
DISCORD_TOKEN = "put-discord-token-here"
```
2. Run `cargo shuttle deploy`
