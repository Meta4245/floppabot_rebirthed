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
use substring::Substring;

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

#[derive(Deserialize)]
struct RandomFox {
    image: String,
    link: String,
}

#[derive(Deserialize)]
struct AWSRandomCat {
    file: String,
}

enum AnimalOnlineChoices {
    Bird,
    Shibe,
    Cat,
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
        .expect("error in json")
}

fn request_fox() -> RandomFox {
    ureq::get("https://randomfox.ca/floof/")
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json")
}

fn request_animalonline(animal: AnimalOnlineChoices) -> String {
    let url = match animal {
        AnimalOnlineChoices::Bird => "http://shibe.online/api/birds",
        AnimalOnlineChoices::Shibe => "http://shibe.online/api/shibes",
        AnimalOnlineChoices::Cat => "http://shibe.online/api/cats",
    };
    let mut shibe = ureq::get(&url)
        .call()
        .expect("error in req")
        .into_string()
        .expect("error in string");
    shibe = shibe.substring(2, shibe.chars().count() - 2).to_string();
    shibe
}

fn request_awscat() -> AWSRandomCat {
    ureq::get("https://aws.random.cat/meow")
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json")
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

#[poise::command(slash_command)]
pub async fn fox_image(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_fox();

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Fox");
                e.image(&parsed.image)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Fox")
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
                    e.title("Random Fox");
                    e.image(&request_fox().image)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Fox")
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
pub async fn shiba_image(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_animalonline(AnimalOnlineChoices::Shibe);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Shiba");
                e.image(&parsed)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Shiba")
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
                    e.title("Random Shiba");
                    e.image(&request_animalonline(AnimalOnlineChoices::Shibe))
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Shiba")
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
pub async fn bird_image(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_animalonline(AnimalOnlineChoices::Bird);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Bird");
                e.image(&parsed)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Bird")
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
                    e.title("Random Bird");
                    e.image(&request_animalonline(AnimalOnlineChoices::Bird))
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Bird")
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
pub async fn cat_image2(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_animalonline(AnimalOnlineChoices::Cat);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Cat");
                e.image(&parsed)
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
                    e.image(&request_animalonline(AnimalOnlineChoices::Cat))
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
pub async fn cat_image3(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_awscat().file;

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Cat");
                e.image(&parsed)
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
                    e.image(&request_awscat().file)
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
