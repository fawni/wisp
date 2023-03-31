use crate::{
    api::fourchan::{Catalog, Post, Thread},
    Context, Error, ACCENT_COLOR,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use nanorand::{Rng, WyRand};

async fn cute_boards<'a>(_ctx: Context<'_>, _partial: &'a str) -> Vec<String> {
    vec![String::from("c"), String::from("cm")]
}

/// get a /cute/ picture
#[poise::command(prefix_command, slash_command)]
pub async fn cute(
    ctx: Context<'_>,
    #[autocomplete = "cute_boards"]
    #[description = "board to pick from"]
    board: Option<String>,
) -> Result<(), Error> {
    let mut rng = WyRand::new();

    let cute_boards = vec!["c", "cm"];
    let board = board
        .unwrap_or_else(|| String::from(cute_boards[rng.generate_range(0..cute_boards.len())]));

    let catalog = reqwest::get(&format!("https://a.4cdn.org/{board}/catalog.json"))
        .await?
        .json::<Vec<Catalog>>()
        .await?;

    let thread_no = catalog[rng.generate_range(0..9)].threads[rng.generate_range(0..14)].no;

    let thread = reqwest::get(&format!(
        "https://a.4cdn.org/{board}/thread/{thread_no}.json"
    ))
    .await?
    .json::<Thread>()
    .await?;

    let posts = thread
        .posts
        .into_iter()
        .filter(|p| p.is_image() && !p.is_sticky())
        .collect::<Vec<Post>>();
    let post = posts[rng.generate_range(0..posts.len())].clone();
    let ext = post.ext.unwrap();
    let image = format!("https://i.4cdn.org/{board}/{}{ext}", post.tim.unwrap());

    ctx.send(|r| {
        r.embed(|r| {
            r.image(image)
                .color(*ACCENT_COLOR)
                .title(format!("No. {}", post.no))
                .description(format!("{}{}", post.filename.unwrap(), ext))
                .author(|a| {
                    a.name(format!("/{board}/"))
                        .icon_url("https://i.imgur.com/XcCKhYj.png")
                        .url(format!("https://boards.4channel.org/{board}/"))
                })
                .footer(|f| {
                    f.text(format!(
                        "{} | {}",
                        post.tim.unwrap(),
                        DateTime::<Utc>::from_utc(
                            NaiveDateTime::from_timestamp_opt(post.time, 0).unwrap(),
                            Utc,
                        )
                        .with_timezone(&chrono_tz::Tz::Africa__Cairo)
                        .format("%I:%M:%S %p • %d %b %Y")
                    ))
                })
        })
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.label("view post")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url(format!(
                            "https://boards.4channel.org/{board}/thread/{thread_no}#p{}",
                            post.no
                        ))
                })
                .create_button(|b| {
                    b.label("view thread")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url(format!(
                            "https://boards.4channel.org/{board}/thread/{thread_no}"
                        ))
                })
            })
        })
    })
    .await?;

    Ok(())
}
