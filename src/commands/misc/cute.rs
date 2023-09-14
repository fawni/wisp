use chrono::{DateTime, NaiveDateTime, Utc};
use nanorand::{Rng, WyRand};
use poise::AutocompleteChoice;

use crate::serenity::ButtonStyle;
use crate::sources::fourchan::{self, Post, Thread};
use crate::{Context, Error, COLOR};

async fn cute_boards<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice<&'a str>> + 'a {
    ["c", "cm"].into_iter().map(|name| AutocompleteChoice {
        name: format!("/{name}/"),
        value: name,
    })
}

/// Get a /cute/ picture
#[poise::command(prefix_command, track_edits, slash_command, aliases("c", "cm"))]
pub async fn cute(
    ctx: Context<'_>,
    #[autocomplete = "cute_boards"]
    #[description = "board to pick from"]
    board: Option<String>,
) -> Result<(), Error> {
    let mut rng = WyRand::new();

    let cute_boards = ["c", "cm"];
    let board = board
        .unwrap_or_else(|| String::from(cute_boards[rng.generate_range(0..cute_boards.len())]));
    let catalog = fourchan::get_catalog(&board).await?;
    let thread_no = catalog[rng.generate_range(0..9)].threads[rng.generate_range(0..14)].no;
    let posts = Thread::from(&board, thread_no)
        .await?
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
                .color(*COLOR)
                .title(format!("No. {}", post.no))
                .description(format!("{}{ext}", post.filename.unwrap()))
                .author(|a| {
                    a.name(format!("/{board}/"))
                        .icon_url("https://i.imgur.com/XcCKhYj.png")
                        .url(format!("https://boards.4channel.org/{board}/"))
                })
                .footer(|f| {
                    f.text(format!(
                        "{} | {}",
                        post.tim.unwrap(),
                        DateTime::<Utc>::from_naive_utc_and_offset(
                            NaiveDateTime::from_timestamp_opt(post.time, 0).unwrap(),
                            Utc
                        )
                        .with_timezone(&chrono_tz::Tz::Africa__Cairo)
                        .format("%I:%M:%S %p • %d %b %Y")
                    ))
                })
        })
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.label("view post").style(ButtonStyle::Link).url(format!(
                        "https://boards.4channel.org/{board}/thread/{thread_no}#p{}",
                        post.no
                    ))
                })
                .create_button(|b| {
                    b.label("view thread").style(ButtonStyle::Link).url(format!(
                        "https://boards.4channel.org/{board}/thread/{thread_no}"
                    ))
                })
            })
        })
    })
    .await?;

    Ok(())
}
