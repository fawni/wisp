use poise::serenity_prelude::{CreateAttachment, Message};
use poise::CreateReply;

use crate::sources::tts::{self, Voice};
use crate::{Context, Error};

use crate::serenity::AutocompleteChoice;

async fn voices<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + 'a {
    Voice::list()
        .into_iter()
        .map(|(name, value)| AutocompleteChoice::new(name, value))
}

/// Generate a TTS audio from text
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    broadcast_typing,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn tts(
    ctx: Context<'_>,
    #[description = "Text to turn into speech"] text: String,
    #[autocomplete = "voices"]
    #[description = "Voice to use"]
    voice: Option<String>,
) -> Result<(), Error> {
    run_tts(
        ctx,
        text,
        voice.unwrap_or(Voice::JapaneseFemale3.value().to_owned()),
    )
    .await?;
    Ok(())
}

#[poise::command(
    context_menu_command = "TTS",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn tts_ctx(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    run_tts(
        ctx,
        message.content,
        Voice::JapaneseFemale3.value().to_owned(),
    )
    .await?;
    Ok(())
}

async fn run_tts(ctx: Context<'_>, text: String, voice: String) -> Result<(), Error> {
    ctx.defer().await?;

    #[allow(deprecated)]
    let data = base64::decode(tts::generate(&text, &voice).await?)?;
    ctx.send(
        CreateReply::default().attachment(CreateAttachment::bytes(data, format!("{text}.mp3"))),
    )
    .await?;

    Ok(())
}
