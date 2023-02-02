use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use tracing::info;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!(
            "'DISCORD_TOKEN' was not found in Secrets.toml (add Secrets.toml in base directory)"
        )
        .into());
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![avatar()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
    Ok(client)
}

#[poise::command(slash_command)]
async fn avatar(ctx: Context<'_>, msg: &serenity::Message) -> CommandResult {
    let avatar = match msg.author.avatar_url() {
        None => {
            ctx.say("Failure acquiring avatar.").await?;
            String::from("")
        }
        Some(url) => url,
    };
    ctx.say(avatar).await?;

    Ok(())
}
