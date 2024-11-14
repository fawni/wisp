use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

use crate::{serenity::Role, Context, Error};

/// Role related commands
#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    subcommands("info"),
    category = "Miscellaneous"
)]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Info about a role
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    track_edits,
    install_context = "User",
    interaction_context = "Guild"
)]
pub async fn info(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .description(format!("<@&{}>", role.id))
                .color(role.colour)
                .fields([
                    ("Name", role.name, true),
                    ("Color", format!("#{}", role.colour.hex()), true),
                    ("Mention", format!("`<@&{}>`", role.id), true),
                    ("Hoisted", role.hoist.to_string(), true),
                    ("Position", role.position.to_string(), true),
                    ("Mentionable", role.mentionable.to_string(), true),
                    ("Managed", role.managed.to_string(), true),
                    ("Permissions", role.permissions.to_string(), false),
                ])
                .thumbnail(format!(
                    "https://dummyimage.com/200x200/{}/{0}.png",
                    role.colour.hex()
                ))
                .footer(CreateEmbedFooter::new(format!("ID: {}", role.id)))
                .timestamp(role.id.created_at()),
        ),
    )
    .await?;
    Ok(())
}
