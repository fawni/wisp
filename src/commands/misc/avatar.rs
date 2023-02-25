use crate::{config::ACCENT_COLOR, Context, Error};
use poise::serenity_prelude::{Color, User};

/// displays a user's avatar
#[poise::command(prefix_command, slash_command, aliases("av", "pfp"))]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "user whose avatar will be displayed"] user: Option<User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let member = match ctx.guild() {
        Some(guild) => match guild.member(ctx, user.id).await {
            Ok(member) => Some(member),
            Err(_) => None,
        },
        None => None,
    };
    let mut description = format!(
        "[default]({}), [user]({})",
        user.default_avatar_url(),
        user.face()
    );
    let mut color = Color::new(*ACCENT_COLOR);
    let avatar = match &member {
        Some(member) => {
            description = match member.avatar_url() {
                Some(server_avatar) => format!(
                    "[default]({}), [server]({}), [user]({})",
                    member.user.default_avatar_url(),
                    server_avatar,
                    member.user.face()
                ),
                None => description,
            };
            if let Some(c) = member.colour(ctx) {
                color = c;
            }
            member.face()
        }
        None => user.face(),
    };

    ctx.send(|r| {
        r.embed(|r| {
            r.author(|a| a.name(user.tag()).icon_url(&avatar))
                .description(description)
                .image(avatar)
                .color(color)
        })
    })
    .await?;
    Ok(())
}
