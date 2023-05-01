use crate::{
    sources::fourchan::{get_catalog, Post, Thread},
    Context, Error,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use nanorand::{Rng, WyRand};

/// Get a webm from /wsg/
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn webm(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let mut rng = WyRand::new();
    let board = "wsg".to_owned();

    let catalog = get_catalog(&board).await?;
    let thread_no = catalog[rng.generate_range(0..9)].threads[rng.generate_range(0..14)].no;

    let thread = Thread::from(&board, thread_no).await?;

    let posts = thread
        .posts
        .into_iter()
        .filter(|p| p.is_webm() && !p.is_sticky())
        .collect::<Vec<Post>>();
    let post = posts[rng.generate_range(0..posts.len())].clone();
    let webm = format!("https://i.4cdn.org/{board}/{}.webm", post.tim.unwrap());

    // ctx.send(|r| r.content(webm)).await?;
    ctx.send(|r| {
        r.embed(|r| {
            r.color(0xE83F80)
                .title(format!("No. {}", post.no))
                .description(format!("{}.webm", post.filename.unwrap()))
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
        .attachment(poise::serenity_prelude::AttachmentType::from(webm.as_str()))
    })
    .await?;

    Ok(())
}
