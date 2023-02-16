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

use crate::{Context, Error};
use figlet_rs::FIGfont;

#[poise::command(slash_command)]
pub async fn ascii(
    ctx: Context<'_>,
    #[description = "The string you want to turn into ASCII"] text: String,
) -> Result<(), Error> {
    let standard_font = FIGfont::standard().expect("figlet font fail");
    let figure = standard_font
        .convert(&text)
        .expect("figlet convertion fail");
    let send = format!("```{}```", figure.to_string());
    ctx.say(send).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn uwuifier(
    ctx: Context<'_>,
    #[description = "The text you want to uwu-ify"] text: String,
) -> Result<(), Error> {
    ctx.say(uwuifier::uwuify_str_sse(&text)).await?;

    Ok(())
}
