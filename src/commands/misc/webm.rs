use chrono::{DateTime, Utc};
use nanorand::{Rng, WyRand};
use poise::serenity_prelude::{
    CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter,
};
use poise::CreateReply;

use crate::sources::fourchan::{self, Post, Thread};
use crate::{Context, Error, COLOR};

/// Get a webm from /wsg/
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    category = "Miscellaneous",
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn webm(ctx: Context<'_>) -> Result<(), Error> {
    let mut rng = WyRand::new();

    let board = "wsg";
    let catalog = fourchan::get_catalog(board).await?;
    let thread_no = catalog[rng.generate_range(0..9)].threads[rng.generate_range(0..14)].no;
    let posts = Thread::from(board, thread_no)
        .await?
        .posts
        .into_iter()
        .filter(|p| p.is_webm() && !p.is_sticky())
        .collect::<Vec<Post>>();
    let post = posts[rng.generate_range(0..posts.len())].clone();
    let webm = format!("https://i.4cdn.org/{board}/{}.webm", post.tim.unwrap());

    ctx.defer().await?;
    // ctx.send(|r| r.content(webm)).await?;
    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::default()
                    .color(*COLOR)
                    .title(format!("No. {}", post.no))
                    .description(format!("{}.webm", post.filename.unwrap()))
                    .author(
                        CreateEmbedAuthor::new(format!("/{board}/"))
                            .icon_url("https://i.imgur.com/XcCKhYj.png")
                            .url(format!("https://boards.4channel.org/{board}/")),
                    )
                    .footer(CreateEmbedFooter::new(format!(
                        "{} | {}",
                        post.tim.unwrap(),
                        DateTime::<Utc>::from_timestamp(post.time, 0)
                            .unwrap()
                            .with_timezone(&chrono_tz::Tz::Africa__Cairo)
                            .format("%I:%M:%S %p • %d %b %Y")
                    ))),
            )
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new_link(format!(
                    "https://boards.4channel.org/{board}/thread/{thread_no}#p{}",
                    post.no
                ))
                .label("view post"),
                CreateButton::new_link(format!(
                    "https://boards.4channel.org/{board}/thread/{thread_no}"
                ))
                .label("view thread"),
            ])])
            .attachment(CreateAttachment::url(ctx.http(), webm.as_str()).await?),
    )
    .await?;

    Ok(())
}
