use crate::{config::ACCENT_COLOR, Context, Error};
use chrono::{DateTime, NaiveDateTime, Utc};
use nanorand::{Rng, WyRand};
use poise::serenity_prelude::AttachmentType;
use rchan::{client::Client, prelude::Post};

/// get a webm from wsg
#[poise::command(prefix_command, slash_command)]
pub async fn webm(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let client = Client::new();
    let mut rng = WyRand::new();

    let board = "wsg";
    let catalog = client.get_board_catalog(board).await?.0;
    let page = &catalog[rng.generate_range(0..catalog.len())];
    let thread_id = page.threads[rng.generate_range(0..page.threads.len())].thread_no();

    let thread = client.get_full_thread(board, thread_id).await?;
    let posts = thread
        .posts
        .into_iter()
        .filter(|p| p.attachment.is_some())
        .collect::<Vec<Post>>();
    let mut post: &Post;

    loop {
        post = &posts[rng.generate_range(0..posts.len())];
        if post.attachment_url(board).is_some()
            && !thread.sticky
            && post.attachment.as_ref().unwrap().ext == ".webm"
        {
            break;
        }
    }

    let webm: &str = &post.attachment_url(board).unwrap();
    let metadata = post.attachment.as_ref().unwrap();

    /* this is fast but ugly; sends two messages */
    // ctx.send(|r| r.content(webm)).await?;
    ctx.send(|r| {
        r.embed(|r| {
            r.color(*ACCENT_COLOR)
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
        /* sends one pretty message but is slow depending on the connection(?) */
        .attachment(AttachmentType::from(webm))
    })
    .await?;

    Ok(())
}
