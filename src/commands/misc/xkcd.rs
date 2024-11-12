use crate::COLOR;
use crate::{
    sources::xkcd::{Comic, BASE_URL},
    Context, Error,
};
use nanorand::{Rng, WyRand};
use poise::serenity_prelude::{
    CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
};
use poise::CreateReply;

/// Get an xkcd comic
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn xkcd(
    ctx: Context<'_>,
    #[description = "specific comic to retrieve"] id: Option<u32>,
) -> Result<(), Error> {
    let latest = Comic::latest().await?.num;

    let id = match id {
        Some(n) if n <= latest => n,
        _ => WyRand::new().generate_range(1..=latest),
    };

    let comic = Comic::from(id).await?;

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::default()
                    .author(
                        CreateEmbedAuthor::new("xkcd")
                            .url(BASE_URL)
                            .icon_url("https://i.imgur.com/7AHZKBD.png"),
                    )
                    .title(format!("**#{}** {}", comic.num, comic.safe_title))
                    .description(comic.alt)
                    .image(comic.img)
                    .color(*COLOR)
                    .footer(CreateEmbedFooter::new(format!(
                        "{}/{}/{}",
                        comic.day, comic.month, comic.year
                    ))),
            )
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new_link(format!("{}/{}", BASE_URL, comic.num))
                    .label("view on xkcd.com"),
            ])]),
    )
    .await?;

    Ok(())
}
