use crate::{
    serenity::{Mentionable, User},
    COLOR,
};
use chrono_tz::Tz;

use crate::{commands::CommandError, Context, Error};

/// Query information about a Discord user
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    guild_only,
    aliases("userinfo", "whois")
)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Discord profile to query information about"] user: Option<User>,
) -> Result<(), Error> {
    run_user(ctx, user).await?;
    Ok(())
}

#[poise::command(context_menu_command = "User Info", guild_only)]
pub async fn user_ctx(ctx: Context<'_>, user: User) -> Result<(), Error> {
    run_user(ctx, Some(user)).await?;
    Ok(())
}

async fn run_user(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let guild = ctx.guild().ok_or(CommandError::GuildOnly)?;
    let member = guild.member(&ctx, user.id).await?;
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

    ctx.send(|r| {
        r.embed(|e| {
            e.author(|a| a.name(user.tag()).icon_url(user.face()))
                .thumbnail(user.face())
                .color(member.colour(ctx).unwrap_or(*COLOR))
                .description(user.mention())
                .fields([
                    ("Joined", joined_at.to_string(), true),
                    ("Created", created_at.to_string(), true),
                    ("Bot", user.bot.to_string(), false),
                    (&format!("Roles [{roles_count}]"), roles, true),
                    ("Permissions", perms, false),
                ])
                .footer(|f| f.text(format!("ID: {}", user.id)))
        })
    })
    .await?;

    Ok(())
}
