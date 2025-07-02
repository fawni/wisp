use poise::{serenity_prelude::CreateAttachment, CreateReply};

use crate::{Context, Error};

/// Ask or chat with an LLM
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn wisp(
    ctx: Context<'_>,
    #[rest]
    #[description = "Question to ask"]
    prompt: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let answer = ctx
        .data()
        .gemini_client
        .generate_response_with_context(&prompt, ctx.author().display_name(), &[], None)
        .await?;

    ctx.reply(answer).await?;

    Ok(())
}

/// Generate an image
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn generate(
    ctx: Context<'_>,
    #[rest]
    #[description = "Prompt to generate"]
    prompt: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let (data, description) = ctx.data().gemini_client.generate_image(&prompt).await?;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "gemini_image_{}.png",
        chrono::Utc::now().timestamp()
    ));

    std::fs::write(&file_path, &data)?;

    let files = vec![CreateAttachment::path(&file_path).await?];

    let message_content = if description.is_empty() {
        format!("Here's what I imagine for: {}", prompt)
    } else {
        format!("Here's what I imagine for: {}\n\n{}", prompt, description)
    };

    let builder = files
        .into_iter()
        .fold(CreateReply::default().content(message_content), |b, f| {
            b.attachment(f)
        });

    ctx.reply_builder(builder);

    std::fs::remove_file(file_path)?;

    Ok(())
}
