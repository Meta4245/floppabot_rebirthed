use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn avatar(ctx: Context<'_>, #[description = "User you want to get avatar of"] user: Option<serenity::User>) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = u.avatar_url().unwrap_or_else(|| String::from("Error getting user avatar"));
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttlePoise<Data, Error> {
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
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("fl.".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![avatar(), register()],
            ..Default::default()
        })
        .token(token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_service::error::CustomError::new)?;

    Ok(framework)
}