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
use poise::serenity_prelude::User;

#[poise::command(slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "User you want to get avatar of"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = u
        .avatar_url()
        .unwrap_or_else(|| String::from("Error getting user avatar"));
    ctx.say(response).await?;

    Ok(())
}
