use crate::{Context, Error};

/// Register application commands
#[poise::command(prefix_command, owners_only, category = "Owner")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
