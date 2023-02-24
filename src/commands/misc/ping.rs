use crate::{Context, Error};
use tokio::time::Instant;

/// ping pong
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.send(|r| r.content("( =ω= ) ")).await?;
    let duration = start.elapsed().as_millis();
    msg.edit(ctx, |r| r.content(format!("( =ω= ) `{duration}ms`")))
        .await?;
    Ok(())
}
