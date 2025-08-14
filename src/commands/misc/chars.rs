use poise::serenity_prelude::Message;

use crate::{sources::unicode::get_unicode_name, Context, Error};

/// Inspect the unicode characters in a string of text
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    broadcast_typing,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn chars(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to inspect its characters"]
    text: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    run_chars(ctx, text).await?;
    Ok(())
}

#[poise::command(
    context_menu_command = "Chars",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn chars_ctx(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    run_chars(ctx, message.content).await?;
    Ok(())
}

pub async fn run_chars(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let mut content = String::new();

    for (c, name) in get_unicode_name(&text).await {
        content.push_str(&format!(
            "``{ZWSP} {c} {ZWSP}`` {name}\n",
            ZWSP = "\u{200B}"
        ));
    }

    ctx.reply(content).await?;
    Ok(())
}
