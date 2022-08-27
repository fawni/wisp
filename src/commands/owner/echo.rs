use poise::serenity_prelude::MessageId;

use crate::{Context, Error};

/// repeat a message
#[poise::command(slash_command, owners_only, ephemeral)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "message id to reply to"] reply_id: Option<String>,
    #[description = "text to echo"] text: String,
) -> Result<(), Error> {
    let channel = ctx.channel_id();
    let mut response = "done!";

    if let Some(reply_id) = reply_id {
        let reply_message = channel
            .message(&ctx.discord().http, MessageId(reply_id.parse::<u64>()?))
            .await;
        if let Ok(msg) = reply_message {
            msg.reply(&ctx.discord().http, &text).await?;
        } else {
            response = "invalid message id to reply to:(";
        };
    } else {
        channel.say(&ctx.discord().http, text).await?;
    };
    ctx.say(response).await?;
    Ok(())
}
