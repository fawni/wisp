use poise::CreateReply;

use crate::{Context, Error};

/// Ping the bot
#[poise::command(prefix_command, track_edits, slash_command, category = "Miscellaneous")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.send(CreateReply::default().content("(๑˃ᴗ˂)ﻭ")).await?;
    let duration = start.elapsed().as_millis();
    msg.edit(
        ctx,
        CreateReply::default().content(format!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ `{duration}ms`")),
    )
    .await?;
    Ok(())
}
