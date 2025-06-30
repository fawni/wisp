use crate::{Context, Error, WOLFRAM};
use poise::CreateReply;

/// Query the WolframAlpha api
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    category = "Miscellaneous",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn wolfram(
    ctx: Context<'_>,
    #[description = "Query for WolframAlpha"] query: String,
) -> Result<(), Error> {
    let res = reqwest::get(format!(
        "http://api.wolframalpha.com/v1/result?appid={}&i={}",
        *WOLFRAM,
        urlencoding::encode(&query)
    ))
    .await?;

    ctx.send(CreateReply::default().content(format!("```{}```", res.text().await?)))
        .await?;

    Ok(())
}
