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

mod commands;
mod secrets;
use commands::*;
use poise::serenity_prelude as serenity;
use secrets::*;
use shuttle_service::ShuttlePoise;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_service::main]
async fn poise() -> ShuttlePoise<Data, Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("fl.".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            on_error: |err| {
                Box::pin(async move {
                    match err {
                        poise::FrameworkError::Command { ctx, .. } => {
                            println!(
                                "An error occured: {:?}",
                                ctx.invocation_data::<&str>().await.as_deref()
                            );
                        }
                        err => poise::builtins::on_error(err).await.unwrap(),
                    }
                })
            },
            commands: vec![images::animals(), info::userinfo()],
            ..Default::default()
        })
        .token(DISCORD_TOKEN)
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
