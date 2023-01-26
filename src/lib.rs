use anyhow::anyhow;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group,};
use serenity::framework::standard::{
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::info;

#[group]
#[commands(avatar)]
struct Info;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("fl."))
        .group(&INFO_GROUP);

    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    Ok(client)
}

#[command]
#[description("Gets the avatar of the person who said the command.")]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let avatar = match msg.author.avatar_url() {
        None => {
            msg.reply(ctx, "Failure acquiring avatar.").await?;
            String::from("")
        }
        Some(url) => url,
    };
    msg.reply(ctx, avatar).await?;

    Ok(())
}