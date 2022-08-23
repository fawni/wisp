use crate::{constants::ACCENT_COLOR, Context, Error};
use poise::serenity_prelude::{self as serenity};
use tokio::time::Instant;

/// ping pong
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.send(|r| r.content("( =ω= ) ")).await?;
    let duration = start.elapsed().as_millis();
    msg.edit(ctx, |r| r.content(format!("( =ω= ) `{}ms`", duration)))
        .await?;
    Ok(())
}

/// displays a user's avatar
#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "user whose avatar will be displayed"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let member = match ctx.guild() {
        Some(guild) => match guild.member(ctx.discord(), user.id).await {
            Ok(member) => Some(member),
            Err(_) => None,
        },
        None => None,
    };
    let mut description = "(=^ ◡ ^=)".to_string();
    let avatar = match &member {
        Some(member) => {
            description = match member.avatar_url() {
                Some(server_avatar) => format!(
                    "[default]({}), [server]({}), [user]({})",
                    member.user.default_avatar_url(),
                    server_avatar,
                    member.user.face()
                ),
                None => format!(
                    "[default]({}), [user]({})",
                    member.user.default_avatar_url(),
                    member.user.face()
                ),
            };
            member.face()
        }
        None => user.face(),
    };

    ctx.send(|r| {
        r.embed(|r| {
            r.author(|a| a.name(user.tag()).icon_url(&avatar))
                .description(description)
                .image(avatar)
                .color(serenity::Color::from(ACCENT_COLOR))
        })
    })
    .await?;
    Ok(())
}
