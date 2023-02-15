// Copyright 2023 EagleOnGitHub
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

use crate::{
    secrets::{CAT_API_KEY, DOG_API_KEY},
    Context, Error,
};
use serde::Deserialize;

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
struct AnimalAPIResponse {
    id: String,
    width: i16,
    height: i16,
    url: String,
    breeds: Option<Vec<Breeds>>,
}

fn request_animalapi(cat: bool) -> Vec<AnimalAPIResponse> {
    let url: String;
    url = match cat {
        true => {
            format!(
                "https://api.thecatapi.com/v1/images/search?api_key={}",
                CAT_API_KEY
            )
        }
        false => {
            format!(
                "https://api.thedogapi.com/v1/images/search?api_key={}",
                DOG_API_KEY
            )
        }
    };
    ureq::get(&url)
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in req")
}

#[poise::command(slash_command)]
pub async fn cat_image(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_animalapi(true);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Cat");
                e.image(&parsed[0].url)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Cat")
                            .custom_id(button_uuid)
                    })
                })
            })
    })
    .await?;

    while let Some(mci) = poise::serenity_prelude::CollectComponentInteraction::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == button_uuid.to_string())
        .await
    {
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Cat");
                    e.image(&request_animalapi(true)[0].url)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Cat")
                                .custom_id(button_uuid)
                        })
                    })
                })
        })
        .await?;

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(poise::serenity_prelude::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn dog_image(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_animalapi(false);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Dog");
                e.image(&parsed[0].url)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Dog")
                            .custom_id(button_uuid)
                    })
                })
            })
    })
    .await?;

    while let Some(mci) = poise::serenity_prelude::CollectComponentInteraction::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == button_uuid.to_string())
        .await
    {
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Dog");
                    e.image(&request_animalapi(false)[0].url)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Dog")
                                .custom_id(button_uuid)
                        })
                    })
                })
        })
        .await?;

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(poise::serenity_prelude::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}
