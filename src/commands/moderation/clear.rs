use crate::{Context, Error};
use poise::serenity_prelude::{Message, User};

/// mass delete messages
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
    amount: u32,
    #[description = "only from user"] user: Option<User>,
) -> Result<(), Error> {
    let mut reply = ctx.say("clearing...").await?.into_message().await?;
    let channel = ctx.channel_id();

    let mut messages = channel
        .messages(&ctx.discord().http, |m| {
            m.limit(amount as u64).before(reply.id)
        })
        .await?;

    // todo: fix clear by user. messages are fetched with no regard for its author then filtered; messages by user deleted < amount desired.
    if let Some(user) = user {
        messages = messages
            .into_iter()
            .filter(|m| m.author.id == user.id)
            .collect::<Vec<Message>>();
    }

    channel
        .delete_messages(&ctx.discord().http, &messages)
        .await?;
    reply
        .edit(ctx.discord(), |r| {
            r.content(format!("done! cleared `{}` messages.", messages.len()))
        })
        .await?;

    Ok(())
}
