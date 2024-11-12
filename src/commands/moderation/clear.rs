use poise::serenity_prelude::{EditMessage, GetMessages};

use crate::serenity::{Message, User};
use crate::{Context, Error};

/// Clear messages
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear(
    ctx: Context<'_>,
    #[min = 1]
    #[max = 1000]
    #[description = "how many messages to delete"]
    amount: u16,
    #[description = "only from user"] user: Option<User>,
) -> Result<(), Error> {
    let mut reply = ctx.say("clearing...").await?.into_message().await?;
    let channel = ctx.channel_id();

    let mut messages = channel
        .messages(
            &ctx,
            GetMessages::default().limit(amount as u8).before(reply.id),
        )
        .await?;

    if let Some(user) = user {
        messages = channel
            .messages(&ctx, GetMessages::default().before(reply.id))
            .await?
            .into_iter()
            .filter(|m| m.author.id == user.id)
            .take(amount as usize)
            .collect::<Vec<Message>>();
    }

    channel.delete_messages(&ctx, &messages).await?;
    reply
        .edit(
            ctx,
            EditMessage::default().content(format!("done! cleared `{}` messages.", messages.len())),
        )
        .await?;

    Ok(())
}
