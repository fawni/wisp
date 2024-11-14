use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor};
use poise::CreateReply;

use crate::serenity::User;
use crate::{Context, Error, COLOR};

/// Display a user's avatar
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    aliases("av", "pfp"),
    category = "Miscellaneous",
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "user whose avatar will be displayed"] user: Option<User>,
) -> Result<(), Error> {
    run_avatar(ctx, user).await?;
    Ok(())
}

#[poise::command(
    context_menu_command = "Avatar",
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn avatar_ctx(ctx: Context<'_>, user: User) -> Result<(), Error> {
    run_avatar(ctx, Some(user)).await?;
    Ok(())
}

async fn run_avatar(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let member = ctx.author_member().await;
    let mut description = format!(
        "[default]({}), [normal]({})",
        user.default_avatar_url(),
        user.face()
    );
    let mut color = *COLOR;
    let avatar = member.map_or_else(
        || user.face(),
        |member| {
            description = match member.avatar_url() {
                Some(server_avatar) => format!(
                    "[default]({}), [server]({}), [normal]({})",
                    member.user.default_avatar_url(),
                    server_avatar,
                    member.user.face()
                ),
                None => description.clone(),
            };
            if let Some(c) = member.colour(ctx) {
                color = c;
            }

            member.face()
        },
    );

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .author(CreateEmbedAuthor::new(user.tag()).icon_url(&avatar))
                .description(description)
                .image(avatar)
                .color(color),
        ),
    )
    .await?;

    Ok(())
}
