use poise::serenity_prelude::Message;

use crate::{Context, Error};

/// Repeat a message
#[poise::command(slash_command, owners_only, ephemeral)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "message id to reply to"] msg: Option<Message>,
    #[description = "text to echo"] text: String,
) -> Result<(), Error> {
    let channel = ctx.channel_id();
    if let Some(msg) = msg {
        channel.message(&ctx, msg).await?.reply(&ctx, &text).await?;
    } else {
        channel.say(&ctx, text).await?;
    };
    ctx.say("done").await?;
    Ok(())
}
