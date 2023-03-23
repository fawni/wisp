use poise::serenity_prelude::{Context, Message};

use crate::Error;

mod tiktok;

pub async fn handle(ctx: Context, msg: Message) -> Result<(), Error> {
    if tiktok::matches(&msg) {
        tiktok::reembed(ctx, msg).await?;
    };

    Ok(())
}
