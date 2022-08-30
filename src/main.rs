use config::PREFIX;
use paris::{info, success};
use poise::serenity_prelude::{self as serenity, Activity, OnlineStatus};
use std::time::Duration;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

mod commands;
mod config;
use commands::{
    misc::{avatar::*, ping::*},
    moderation::clear::*,
    owner::{echo::*, register::*},
};

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        poise::Event::Ready {
            data_about_bot: bot,
        } => {
            ctx.set_presence(
                Some(Activity::listening("you, cutie <3")),
                OnlineStatus::DoNotDisturb,
            )
            .await;
            success!("<bold>{}</> is <green>connected!</>", bot.user.name);
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    kankyo::init()?;
    color_eyre::install()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), avatar(), echo(), clear()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(PREFIX.to_string()),
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                ..Default::default()
            },
            post_command: |ctx| {
                Box::pin(async move {
                    let location = match ctx.guild() {
                        Some(guild) => {
                            format!("<magenta>{}</> (<italic>{}</>)", guild.name, guild.id)
                        }
                        None => format!("<magenta>{}'s dms</>", ctx.author().tag()),
                    };
                    info!(
                        "executed <green>{}</> by <bold>{}</> (<italic>{}</>) in {}",
                        ctx.command().qualified_name,
                        ctx.author().tag(),
                        ctx.author().id,
                        location,
                    );
                })
            },
            listener: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN env var"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await?;

    Ok(())
}
