use crate::{sources::wolfram::Wolfram, Context, Error};
use poise::CreateReply;

/// Query the WolframAlpha api
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn wolfram(
    ctx: Context<'_>,
    #[rest]
    #[description = "Query for WolframAlpha"]
    query: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let answer = Wolfram::query(query).await?;

    ctx.send(CreateReply::default().content(format!("```{answer}```")))
        .await?;

    Ok(())
}
