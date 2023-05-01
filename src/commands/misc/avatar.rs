use crate::{Context, Error};
use poise::serenity_prelude::{Color, User};

/// Display a user's avatar
#[poise::command(prefix_command, track_edits, slash_command, aliases("av", "pfp"))]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "user whose avatar will be displayed"] user: Option<User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().to_owned());
    let member = match ctx.guild() {
        Some(guild) => (guild.member(ctx, user.id).await).ok(),
        None => None,
    };
    let mut description = format!(
        "[default]({}), [user]({})",
        user.default_avatar_url(),
        user.face()
    );
    let mut color = Color::new(0xE83F80);
    let avatar = member.map_or_else(
        || user.face(),
        |member| {
            description = match member.avatar_url() {
                Some(server_avatar) => format!(
                    "[default]({}), [server]({}), [user]({})",
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
