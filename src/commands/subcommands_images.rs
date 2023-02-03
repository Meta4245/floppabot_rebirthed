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

use crate::{Context, Error, secrets::CAT_API_KEY};
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
struct CatAPIResponse {
    id: String,
    width: String,
    height: String,
    url: String,
    breeds: Vec<Breeds>,
}

pub fn handle_option(handle: &Option<String>) -> String {
    match handle {
        Some(value) => value.to_string(),
        None => String::from("No Info"),
    }
}

#[poise::command(slash_command)]
pub async fn cat_image(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", CAT_API_KEY)
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