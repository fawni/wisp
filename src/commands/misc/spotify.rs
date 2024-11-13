use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor};
use poise::CreateReply;

use crate::serenity::{ActivityType, User};
use crate::{commands::CommandError, Context, Error};

/// Display info about a user's currently playing Spotify song
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    guild_only,
    category = "Miscellaneous"
)]
pub async fn spotify(
    ctx: Context<'_>,
    #[description = "user whose Spotify status will be checked"] user: Option<User>,
) -> Result<(), Error> {
    run_spotify(ctx, user).await
}

#[poise::command(context_menu_command = "Spotify", guild_only)]
pub async fn spotify_ctx(ctx: Context<'_>, user: User) -> Result<(), Error> {
    run_spotify(ctx, Some(user)).await
}

async fn run_spotify(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let activities = {
        let guild = ctx.guild().ok_or(CommandError::GuildOnly)?;

        guild
            .presences
            .get(&user.id)
            .ok_or(CommandError::PresenceNotFound)?
            .activities
            .clone()
    };
    let spotify = activities
        .iter()
        .find(|a| a.kind == ActivityType::Listening && a.name == "Spotify")
        .ok_or(CommandError::NoSpotify)?;
    let timestamps = spotify.timestamps.clone().unwrap();
    // hatred
    let mut start = timestamps.start.unwrap().to_string();
    start.truncate(10);
    let mut end = timestamps.end.unwrap().to_string();
    end.truncate(10);
    let cover = spotify
        .assets
        .clone()
        .unwrap()
        .large_image
        .unwrap()
        .replace("spotify:", "https://i.scdn.co/image/");

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .author(CreateEmbedAuthor::new(user.tag()).icon_url(user.face()))
                .title(format!("**{}**", spotify.details.clone().unwrap()))
                .description(format!(
                    "by **{}**\n on **{}**",
                    spotify.state.clone().unwrap(),
                    spotify.assets.clone().unwrap().large_text.unwrap(),
                ))
                .thumbnail(cover)
                .fields([
                    (
                        "Start",
                        format!("<t:{}:R>", start.parse::<u64>().unwrap()),
                        true,
                    ),
                    (
                        "End",
                        format!("<t:{}:R>", end.parse::<u64>().unwrap()),
                        true,
                    ),
                ])
                .color(0xA3B_AA9),
        ),
    )
    .await?;

    Ok(())
}
