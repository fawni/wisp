use crate::{api::xkcd::Comic, Context, Error, ACCENT_COLOR};
use nanorand::{Rng, WyRand};

const BASE_URL: &str = "https://xkcd.com";
const LATEST_URL: &str = "https://xkcd.com/info.0.json";

/// retrieve an xkcd comic
#[poise::command(prefix_command, slash_command)]
pub async fn xkcd(
    ctx: Context<'_>,
    #[description = "specific comic to retrieve"] id: Option<u32>,
) -> Result<(), Error> {
    let latest = ureq::get(LATEST_URL).call()?.into_json::<Comic>()?.num;
    let n = match id {
        Some(n) if n <= latest => n,
        _ => WyRand::new().generate_range(1..=latest),
    };

    let comic = ureq::get(&format!("{BASE_URL}/{n}/info.0.json"))
        .call()?
        .into_json::<Comic>()?;

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
            .color(*ACCENT_COLOR)
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
