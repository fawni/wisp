use chrono::{DateTime, Utc};
use nanorand::{Rng, WyRand};
use poise::serenity_prelude::{
    CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
};
use poise::CreateReply;

use crate::serenity::AutocompleteChoice;
use crate::sources::fourchan::{self, Post, Thread};
use crate::{Context, Error, COLOR};

async fn cute_boards<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + 'a {
    ["c", "cm"]
        .into_iter()
        .map(|name| AutocompleteChoice::new(format!("/{name}/"), name))
}

/// Get a /cute/ picture
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    aliases("c", "cm"),
    category = "Miscellaneous",
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
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

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::default()
                    .image(image)
                    .color(*COLOR)
                    .title(format!("No. {}", post.no))
                    .description(format!("{}{ext}", post.filename.unwrap()))
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
            ])]),
    )
    .await?;

    Ok(())
}
