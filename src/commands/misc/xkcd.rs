use crate::{config::ACCENT_COLOR, Context, Error};
use nanorand::{Rng, WyRand};
use reqwest::Client;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Comic {
    month: String,
    num: u16,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

const BASE_URL: &str = "https://xkcd.com";
const LATEST_URL: &str = "https://xkcd.com/info.0.json";

/// retrieve an xkcd comic
#[poise::command(prefix_command, slash_command)]
pub async fn xkcd(ctx: Context<'_>) -> Result<(), Error> {
    let client = Client::new();
    let mut rng = WyRand::new();
    let latest = client
        .get(LATEST_URL)
        .send()
        .await?
        .json::<Comic>()
        .await?
        .num;
    let comic = client
        .get(format!(
            "{}/{}/info.0.json",
            BASE_URL,
            rng.generate_range(1..latest)
        ))
        .send()
        .await?
        .json::<Comic>()
        .await?;

    ctx.send(|r| {
        r.embed(|r| {
            r.author(|a| {
                a.name(comic.safe_title)
                    .url(format!("{}/{}", BASE_URL, comic.num))
            })
            .title(format!("**#{}**", comic.num))
            .description(comic.alt)
            .image(comic.img)
            .color(*ACCENT_COLOR)
        })
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.label("view on page")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url(format!("{}/{}", BASE_URL, comic.num))
                })
            })
        })
    })
    .await?;

    Ok(())
}
