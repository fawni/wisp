use poise::{serenity_prelude::CreateAttachment, CreateReply};

use crate::{sources::gemini::GeminiClient, Context, Error, GEMINI_KEY, GEMINI_PROMPT};

/// Ask Gemini AI
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn ask(
    ctx: Context<'_>,
    #[rest]
    #[description = "Question to ask"]
    prompt: String,
) -> Result<(), Error> {
    let client = GeminiClient::new(
        GEMINI_KEY.clone(),
        None,
        Some(GEMINI_PROMPT.clone()),
        "wisp".to_owned(),
    );

    ctx.defer().await?;
    // let answer = client.generate_content(&prompt).await?;
    let answer = client
        .generate_response_with_context(&prompt, ctx.author().display_name(), &vec![], None)
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
    let client = GeminiClient::new(
        GEMINI_KEY.clone(),
        None,
        Some(GEMINI_PROMPT.clone()),
        "wisp".to_owned(),
    );

    ctx.defer().await?;
    let (data, description) = client.generate_image(&prompt).await?;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "gemini_image_{}.png",
        chrono::Utc::now().timestamp()
    ));

    // Write the image data to the file
    std::fs::write(&file_path, &data)?;

    // Create the attachment
    let files = vec![CreateAttachment::path(&file_path).await?];

    // Format the message with both the prompt and the AI's description
    let message_content = if description.is_empty() {
        format!("Here's what I imagine for: {}", prompt)
    } else {
        format!("Here's what I imagine for: {}\n\n{}", prompt, description)
    };

    // Send the image file with the description
    let builder = files
        .into_iter()
        .fold(CreateReply::default().content(message_content), |b, f| {
            b.attachment(f)
        });

    ctx.reply_builder(builder);

    // Send the message

    // Clean up the temporary file
    std::fs::remove_file(file_path)?;

    Ok(())
}
