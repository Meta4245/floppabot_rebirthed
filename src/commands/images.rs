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
    commands::subcommands_images::{
        bird, cat, cat2, cat3, dog, dog2, duck, fox, httpcat, httpdog, kitsune, neko, neko2, neko3,
        okami, shiba,
    },
    Context, Error,
};

#[poise::command(
    slash_command,
    subcommands(
        "cat", "dog", "fox", "shiba", "bird", "cat2", "cat3", "httpcat", "httpdog", "duck", "dog2"
    )
)]
pub async fn animals(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    subcommands("neko", "neko2", "neko3", "kitsune", "okami")
)]
pub async fn anime(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
