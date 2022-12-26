use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod oot;
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.attachments.len() != 1 || msg.author.bot {
            return;
        }

        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Takk! Vent litt, mens tørrfesken tørka...")
            .await
        {
            error!("Error sending message: {:?}", why);
            return;
        }

        let attachment = msg.attachments.get(0).unwrap();
        match attachment.download().await {
            Ok(content) => {
                let json = String::from_utf8(content).ok().unwrap();
                let spoiler_log = oot::parse_spoiler_log(json);
                let files = vec![(spoiler_log.as_bytes(), "spoiler-log.txt")];

                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, files, |m| m.content("Her e spoiler loggen!"))
                    .await
                {
                    error!("Error sending message: {:?}", why);
                    return;
                }
            }
            Err(why) => {
                error!("Error downloading attachment: {:?}", why);
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Tørrfesken vart blaut. Kontakt support!");

                return;
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
