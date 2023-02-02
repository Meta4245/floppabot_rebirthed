// Copyright 2023 Meta4245
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(dead_code, non_snake_case)]
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use shuttle_secrets::SecretStore;
use shuttle_service::ShuttlePoise;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Deserialize)]
struct Breeds {
    weight: Option<Vec<String>>,
    id: Option<String>,
    name: Option<String>,
    temperament: Option<String>,
    origin: Option<String>,
    country_codes: Option<String>,
    country_code: Option<String>,
    life_span: Option<String>,
    wikipedia_url: Option<String>,
}

#[derive(Deserialize)]
struct CatAPIResponse {
    id: String,
    width: String,
    height: String,
    url: String,
    breeds: Vec<Breeds>,
}

#[derive(Deserialize)]
struct Secrets {
    DISCORD_TOKEN: String,
    CAT_API_KEY: String,
    DOG_API_KEY: String,
}

fn handle_option(handle: &Option<String>) -> String {
    match handle {
        Some(value) => value.to_string(),
        None => String::from("No Info"),
    }
}

#[poise::command(slash_command)]
async fn avatar(
    ctx: Context<'_>,
    #[description = "User you want to get avatar of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = u
        .avatar_url()
        .unwrap_or_else(|| String::from("Error getting user avatar"));
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, subcommands("cat_image"))]
async fn animals(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
async fn cat_image(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let root = project_root::get_project_root()?;
    let toml_file = std::fs::read_to_string(root)?;
    let secrets: Secrets = toml::from_str(toml_file.as_str())?;
    let response = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", secrets.CAT_API_KEY)
        .send()
        .await?
        .text()
        .await?;
    let parsed: CatAPIResponse = serde_json::from_str(response.as_str())?;
    let no_info: &mut Vec<String> = &mut Vec::new();
    no_info.push(String::from(""));
    no_info.push(String::from("No Info"));
    let weight = match &parsed.breeds[0].weight {
        Some(a) => a,
        None => no_info,
    };
    ctx.send(|m| {
        m.content("").embed(|e| {
            e.title("Random Cat")
                .image(parsed.url)
                .field("Breed Name", &weight[1], true)
                .field("Weight", handle_option(&parsed.breeds[0].name), true)
                .field(
                    "Temperament",
                    handle_option(&parsed.breeds[0].temperament),
                    true,
                )
                .field(
                    "Origin Country",
                    handle_option(&parsed.breeds[0].origin),
                    true,
                )
                .field("Lifespan", handle_option(&parsed.breeds[0].life_span), true)
                .field(
                    "Wikipedia URL",
                    handle_option(&parsed.breeds[0].wikipedia_url),
                    true,
                )
        })
    })
    .await?;

    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[shuttle_service::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("fl.".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![avatar(), register(), animals()],
            ..Default::default()
        })
        .token(discord_token)
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
