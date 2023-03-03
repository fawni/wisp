use crate::{Context, Error};

async fn commands<'a>(_ctx: Context<'_>, _partial: &'a str) -> Vec<String> {
    vec![String::from("shutdown"), String::from("reboot")]
}

/// ptolemaea
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn ptolemaea(
    ctx: Context<'_>,
    #[autocomplete = "commands"]
    #[description = "command to execute"]
    cmd: String,
) -> Result<(), Error> {
    match cmd.as_str() {
        "shutdown" => {
            ctx.say("Shutting down...").await?;
            system_shutdown::shutdown()?;
        }
        "reboot" => {
            ctx.say("Rebooting...").await?;
            system_shutdown::reboot()?;
        }
        _ => return Err(Error::from("Invalid command")),
    }
    Ok(())
}
