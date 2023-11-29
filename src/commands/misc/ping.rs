use crate::{Context, Error};

/// Ping the bot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.send(|r| r.content("(๑˃ᴗ˂)ﻭ")).await?;
    let duration = start.elapsed().as_millis();
    msg.edit(ctx, |r| r.content(format!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ `{duration}ms`")))
        .await?;
    Ok(())
}
