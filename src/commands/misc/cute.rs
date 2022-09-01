use crate::{config::ACCENT_COLOR, Context, Error};
use chrono::{DateTime, NaiveDateTime, Utc};
use rand::seq::SliceRandom;
use rchan::{client::Client, prelude::Post, thread::Thread};

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

    let cute_boards = vec!["c", "cm"];
    let board = match board {
        Some(b) => b.to_owned(),
        None => cute_boards
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string(),
    };

    let catalog = client.get_board_catalog(board.as_str()).await?.0;
    let page = catalog.choose(&mut rand::thread_rng()).unwrap();
    let thread_id = page
        .threads
        .choose(&mut rand::thread_rng())
        .unwrap()
        .thread_no();

    let mut posts: Vec<Post>;
    let mut thread: Thread;
    loop {
        thread = client.get_full_thread(board.as_str(), thread_id).await?;
        posts = thread
            .posts
            .into_iter()
            .filter(|p| p.attachment.is_some())
            .collect::<Vec<Post>>();

        if !posts.is_empty() && !thread.sticky {
            break;
        }
    }

    let post = posts.choose(&mut rand::thread_rng()).unwrap();
    let image = post.attachment_url(board.as_str()).unwrap();
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
