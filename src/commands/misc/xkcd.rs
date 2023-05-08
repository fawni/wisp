use crate::{
    sources::xkcd::{Comic, BASE_URL},
    Context, Error,
};
use nanorand::{Rng, WyRand};

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

    ctx.send(|r| {
        r.embed(|r| {
            r.author(|a| {
                a.name("xkcd")
                    .url(BASE_URL)
                    .icon_url("https://i.imgur.com/7AHZKBD.png")
            })
            .title(format!("**#{}** {}", comic.num, comic.safe_title))
            .description(comic.alt)
            .image(comic.img)
            .color(0xE83_F80)
            .footer(|f| f.text(format!("{}/{}/{}", comic.day, comic.month, comic.year)))
        })
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.label("view on xkcd.com")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url(format!("{}/{}", BASE_URL, comic.num))
                })
            })
        })
    })
    .await?;

    Ok(())
}
