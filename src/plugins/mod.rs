use crate::serenity::{Context, Message};

use crate::Error;

mod tiktok;

pub async fn handle_message(ctx: Context, msg: Message) -> Result<(), Error> {
    if tiktok::matches(&msg) {
        tiktok::reembed(ctx, msg).await?;
    };

    Ok(())
}
