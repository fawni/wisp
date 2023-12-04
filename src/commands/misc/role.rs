use crate::{serenity::Role, Context, Error};

/// Role
#[poise::command(prefix_command, slash_command, track_edits, subcommands("info"))]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Info about a role
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn info(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.send(|m| {
        m.embed(|e| {
            e.description(format!("<@&{}>", role.id))
                .color(role.colour)
                .fields([
                    ("Name", role.name, true),
                    ("Color", format!("#{}", role.colour.hex()), true),
                    ("Mention", format!("`<@&{}>`", role.id), true),
                    ("Hoisted", role.hoist.to_string(), true),
                    ("Postion", role.position.to_string(), true),
                    ("Mentionable", role.mentionable.to_string(), true),
                    ("Managed", role.managed.to_string(), true),
                    ("Permissions", role.permissions.to_string(), false),
                ])
                .thumbnail(format!(
                    "https://dummyimage.com/200x200/{}/{0}.png",
                    role.colour.hex()
                ))
                .footer(|f| {
                    f.text(format!("ID: {}", role.id))
                        .icon_url(role.icon.unwrap_or_default())
                })
                .timestamp(role.id.created_at())
        })
    })
    .await?;
    Ok(())
}
