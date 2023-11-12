use std::process::Command;

use crate::{Context, Error, COLOR};

async fn commands<'a>(_ctx: Context<'_>, _partial: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    ["shutdown", "reboot"].into_iter()
}

/// ptolemaea
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn sys(
    ctx: Context<'_>,
    #[autocomplete = "commands"]
    #[description = "command to execute"]
    cmd: String,
) -> Result<(), Error> {
    match cmd.as_str() {
        "shutdown" => {
            ctx.send(|r| r.embed(|e| e.description("Shutting down...").color(*COLOR)))
                .await?;
            Command::new("shutdown").args(["/s", "/t", "0"]).spawn()?;
        }
        "reboot" => {
            ctx.send(|r| r.embed(|e| e.description("Rebooting...").color(*COLOR)))
                .await?;
            Command::new("shutdown").args(["/r", "/t", "0"]).spawn()?;
        }
        _ => return Err(Error::from("Invalid command")),
    }
    Ok(())
}
