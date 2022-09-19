// todo: use simple reqwests, remove rchan

use crate::{config::ACCENT_COLOR, Context, Error};
use chrono::{DateTime, NaiveDateTime, Utc};
use nanorand::{Rng, WyRand};
use rchan::{client::Client, prelude::Post};

async fn cute_boards<'a>(_ctx: Context<'_>, _partial: &'a str) -> Vec<String> {
    vec!["c".to_string(), "cm".to_string()]
}

/// get a /cute/ picture
#[poise::command(prefix_command, slash_command)]
pub async fn cute(
    ctx: Context<'_>,
    #[autocomplete = "cute_boards"]
    #[description = "board to open"]
    board: Option<String>,
) -> Result<(), Error> {
    let client = Client::new();
    let mut rng = WyRand::new();

    let cute_boards = vec!["c", "cm"];
    let board = match board {
        Some(b) => b,
        None => cute_boards[rng.generate_range(0..cute_boards.len())].to_string(),
    };

    let catalog = client.get_board_catalog(&board).await?.0;
    let page = &catalog[rng.generate_range(0..catalog.len())];
    let thread_id = page.threads[rng.generate_range(0..page.threads.len())].thread_no();

    let thread = client.get_full_thread(&board, thread_id).await?;
    let posts = thread
        .posts
        .into_iter()
        .filter(|p| p.attachment.is_some())
        .collect::<Vec<Post>>();
    let mut post: &Post;

    loop {
        post = &posts[rng.generate_range(0..posts.len())];
        if post.attachment_url(&board).is_some() && !thread.sticky {
            break;
        }
    }

    let image = post.attachment_url(&board).unwrap();
    let metadata = post.attachment.as_ref().unwrap();

    ctx.send(|r| {
        r.embed(|r| {
            r.image(image)
                .color(*ACCENT_COLOR)
                .title(format!("No. {}", post.no))
                .description(format!("{}{}", metadata.filename, metadata.ext))
                .author(|a| {
                    a.name(format!("/{board}/"))
                        .icon_url("https://i.imgur.com/XcCKhYj.png")
                        .url(format!("https://boards.4channel.org/{}/", board))
                })
                .footer(|f| {
                    f.text(format!(
                        "{} | {}",
                        metadata.id,
                        DateTime::<Utc>::from_utc(
                            NaiveDateTime::from_timestamp(
                                format!("{:.10}", metadata.id.to_string())
                                    .parse::<i64>()
                                    .unwrap(),
                                0,
                            ),
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
                            "https://boards.4channel.org/{}/thread/{}#p{}",
                            board, thread.no, post.no
                        ))
                })
                .create_button(|b| {
                    b.label("view thread")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url(format!(
                            "https://boards.4channel.org/{}/thread/{}",
                            board, thread.no
                        ))
                })
            })
        })
    })
    .await?;

    Ok(())
}
