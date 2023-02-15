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
        bird_image, cat_image, cat_image2, cat_image3, dog_image, fox_image, shiba_image,
    },
    Context, Error,
};

#[poise::command(
    slash_command,
    subcommands(
        "cat_image",
        "dog_image",
        "fox_image",
        "shiba_image",
        "bird_image",
        "cat_image2",
        "cat_image3"
    )
)]
pub async fn animals(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
