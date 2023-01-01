use anyhow::anyhow;
use serde_json::json;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod oot;
struct Bot;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CHANNEL_ID: u64 = 1017171112901218334;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.channel_id.as_u64().eq(&CHANNEL_ID) {
            return;
        }

        if msg.author.bot {
            return;
        }

        if msg.content.eq("!siggud") {
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Siggurd version {}\n    !ignore - Vis ignore-list", VERSION),
                )
                .await
            {
                error!("Error sending message: {:?}", why);
                return;
            }
        }

        if msg.content.eq("!ignore") {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("{}", oot::ignoredKeys().join("\n")))
                .await
            {
                error!("Error sending message: {:?}", why);
                return;
            }
        }

        if msg.attachments.len() != 1 {
            return;
        }

        let attachment = msg.attachments.get(0).unwrap();
        if !attachment.filename.ends_with("Spoiler.json") {
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let message = format!("Siggurd online! Version {}", VERSION);
        // let json: serde_json::Value = serde_json::Value::String(message);
        let json: serde_json::Value = json!({ "content": message });

        if let Err(why) = ctx.http.send_message(CHANNEL_ID, &json).await {
            error!("Error sending message: {:?}", why);
            return;
        }
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
