use crate::{
    serenity::{Mentionable, User},
    COLOR,
};
use chrono_tz::Tz;
use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    CreateReply,
};

use crate::{commands::CommandError, Context, Error};

/// User related commands
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    subcommands("info"),
    aliases("userinfo", "whois"),
    category = "Miscellaneous"
)]
pub async fn user(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    run_user_info(ctx, user).await
}

/// Query information about a Discord user
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    guild_only,
    install_context = "Guild|User",
    interaction_context = "Guild"
)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "User to query information about"] user: Option<User>,
) -> Result<(), Error> {
    run_user_info(ctx, user).await
}

#[poise::command(context_menu_command = "User Info", guild_only)]
pub async fn user_info_ctx(ctx: Context<'_>, user: User) -> Result<(), Error> {
    run_user_info(ctx, Some(user)).await
}

async fn run_user_info(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let guild = ctx.partial_guild().await.ok_or(CommandError::GuildOnly)?;
    let member = ctx.author_member().await.ok_or(CommandError::GuildOnly)?;
    let created_at = user
        .created_at()
        .with_timezone(&Tz::Africa__Cairo)
        .format("%d %b %Y, %I:%M:%S %p");
    let joined_at = member
        .joined_at
        .unwrap()
        .with_timezone(&Tz::Africa__Cairo)
        .format("%d %b %Y, %I:%M:%S %p");

    let perms = member.permissions(ctx).map_or_else(
        |_| "None".to_owned(),
        |perms| {
            if user.id == guild.owner_id {
                "Owner".to_owned()
            } else if perms.administrator() {
                "Administrator".to_owned()
            } else if perms.is_empty() {
                "None".to_owned()
            } else {
                perms.to_string()
            }
        },
    );

    let (roles, roles_count) = member.roles(ctx).map_or_else(
        || ("None".to_string(), 0),
        |roles| {
            (
                if roles.is_empty() {
                    "None".to_string()
                } else {
                    roles
                        .iter()
                        .map(|r| r.mention().to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                },
                roles.len(),
            )
        },
    );

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .author(CreateEmbedAuthor::new(user.tag()).icon_url(user.face()))
                .thumbnail(user.face())
                .color(member.colour(ctx).unwrap_or(*COLOR))
                .description(user.mention().to_string())
                .fields([
                    ("Joined", joined_at.to_string(), true),
                    ("Created", created_at.to_string(), true),
                    ("Bot", user.bot.to_string(), false),
                    (&format!("Roles [{roles_count}]"), roles, true),
                    ("Permissions", perms, false),
                ])
                .footer(CreateEmbedFooter::new(format!("ID: {}", user.id))),
        ),
    )
    .await?;

    Ok(())
}
