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

#[derive(Deserialize)]
struct RandomDog {
    #[serde(rename = "fileSizeBytes")]
    file_size_bytes: i128,
    url: String,
}

#[derive(Deserialize)]
struct RandomDuck {
    url: String,
    message: String,
}

#[derive(Deserialize)]
struct PurrbotImage {
    error: bool,
    link: String,
    time: i16,
}

#[derive(Deserialize)]
struct WaifuPics {
    url: String,
}

enum AnimalOnlineChoices {
    Bird,
    Shibe,
    Cat,
}

enum PurrbotImageChoices {
    Kitsune,
    Neko,
    Okami,
}

impl PurrbotImageChoices {
    fn as_str(&self) -> &'static str {
        match self {
            PurrbotImageChoices::Kitsune => "kitsune",
            PurrbotImageChoices::Okami => "okami",
            PurrbotImageChoices::Neko => "neko",
        }
    }
}

impl AnimalOnlineChoices {
    fn as_str(&self) -> &'static str {
        match self {
            AnimalOnlineChoices::Bird => "birds",
            AnimalOnlineChoices::Shibe => "shibes",
            AnimalOnlineChoices::Cat => "cats",
        }
    }
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
    let url = format!("http://shibe.online/api/{}", animal.as_str());
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

fn request_randomdog() -> RandomDog {
    ureq::get("https://random.dog/woof.json")
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json")
}

fn request_randomduck() -> RandomDuck {
    ureq::get("https://random-d.uk/api/v2/random")
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json")
}

fn request_purrbot(choice: PurrbotImageChoices) -> PurrbotImage {
    let url = format!("https://purrbot.site/api/img/sfw/{}/img", choice.as_str());
    ureq::get(url.as_str())
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json")
}

fn request_waifupics(neko: bool) -> String {
    let complete = match neko {
        true => "neko",
        false => "waifu",
    };
    let url = format!("https://api.waifu.pics/sfw/{}", complete);
    let response: WaifuPics = ureq::get(url.as_str())
        .call()
        .expect("error in req")
        .into_json()
        .expect("error in json");
    response.url
}

// animals

#[poise::command(slash_command)]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn fox(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn shiba(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn bird(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn cat2(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn cat3(ctx: Context<'_>) -> Result<(), Error> {
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

#[poise::command(slash_command)]
pub async fn duck(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_randomduck();

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Duck");
                e.image(&parsed.url)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Duck")
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
                    e.title("Random Duck");
                    e.image(&request_randomduck().url)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Duck")
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
pub async fn dog2(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_randomdog();

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Dog");
                e.image(&parsed.url)
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
                    e.image(&request_randomdog().url)
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
pub async fn httpcat(
    ctx: Context<'_>,
    #[description = "The HTTP code"] code: i16,
) -> Result<(), Error> {
    let send = format!("https://http.cat/{}", code);
    ctx.say(send).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn httpdog(
    ctx: Context<'_>,
    #[description = "The HTTP code"] code: i16,
) -> Result<(), Error> {
    let send = format!("https://http.dog/{}", code);
    ctx.say(send).await?;
    Ok(())
}

// anime

#[poise::command(slash_command)]
pub async fn neko(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = nekosbest::get(nekosbest::Category::Neko).await?;

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Neko");
                e.image(&parsed.url)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Neko")
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
        let response = nekosbest::get(nekosbest::Category::Neko).await?.url;
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Neko");
                    e.image(&response)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Neko")
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
pub async fn neko2(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_waifupics(true);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Neko");
                e.image(&parsed)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Neko")
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
                    e.title("Random Neko");
                    e.image(&request_waifupics(true))
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Neko")
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
pub async fn neko3(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_purrbot(PurrbotImageChoices::Neko);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Neko");
                e.image(&parsed.link);
                e.field("API Response Time", &parsed.time, true)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Neko")
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
        let response = request_purrbot(PurrbotImageChoices::Neko);
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Neko");
                    e.image(&response.link);
                    e.field("API Response Time", &response.time, true)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Neko")
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
pub async fn okami(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_purrbot(PurrbotImageChoices::Okami);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Okami");
                e.image(&parsed.link);
                e.field("API Response Time", &parsed.time, true)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Okami")
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
        let response = request_purrbot(PurrbotImageChoices::Okami);
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Okami");
                    e.image(&response.link);
                    e.field("API Response Time", &response.time, true)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Okami")
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
pub async fn kitsune(ctx: Context<'_>) -> Result<(), Error> {
    let button_uuid = ctx.id();
    let parsed = request_purrbot(PurrbotImageChoices::Kitsune);

    ctx.send(|m| {
        m.content("")
            .embed(|e| {
                e.title("Random Kitsune");
                e.image(&parsed.link);
                e.field("API Response Time", &parsed.time, true)
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(poise::serenity_prelude::ButtonStyle::Primary)
                            .label("New Kitsune")
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
        let response = request_purrbot(PurrbotImageChoices::Kitsune);
        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content("")
                .embed(|e| {
                    e.title("Random Kitsune");
                    e.image(&response.link);
                    e.field("API Response Time", &response.time, true)
                })
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(poise::serenity_prelude::ButtonStyle::Primary)
                                .label("New Kitsune")
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
