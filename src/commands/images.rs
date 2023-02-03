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

use crate::{Context, Error, commands::subcommands_images::cat_image};

#[poise::command(slash_command, subcommands("cat_image"))]
pub async fn animals(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}