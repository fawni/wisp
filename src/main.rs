use std::time::Duration;

use paris::{info, success};
use poise::serenity_prelude::{self as serenity, Activity, OnlineStatus};

use commands::{
    misc::{avatar::avatar, cute::cute, ping::ping, webm::webm, xkcd::xkcd},
    moderation::clear::clear,
    owner::{echo::echo, ptolemaea::ptolemaea, register::register},
};
use lazy_static::lazy_static;
use serde::Deserialize;

mod api;
mod commands;
mod modules;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

lazy_static! {
    pub static ref PREFIX: String = std::env::var("WISP_PREFIX").unwrap();
    pub static ref ACCENT_COLOR: u32 =
        u32::from_str_radix(&std::env::var("WISP_COLOR").unwrap(), 16).unwrap();
}

pub struct Data {}

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    if let poise::Event::Message { new_message: msg } = event {
        modules::handle(ctx.clone(), msg.clone()).await?;
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub accent_color: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let framework = poise::Framework::builder()
        .token(std::env::var("WISP_TOKEN").expect("missing WISP_TOKEN env var"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .options(poise::FrameworkOptions {
            commands: vec![
                register(),
                ping(),
                avatar(),
                echo(),
                clear(),
                cute(),
                webm(),
                xkcd(),
                ptolemaea(),
            ],
            post_command: |ctx| {
                Box::pin(async move {
                    let location = if let Some(guild) = ctx.guild() {
                        if let Some(channel) = ctx.channel_id().name(&ctx).await {
                            format!(
                                "<magenta>#{}, {}</> (<italic>{}</>)",
                                channel, guild.name, guild.id
                            )
                        } else {
                            format!("<magenta>{}</> (<italic>{}</>)", guild.name, guild.id)
                        }
                    } else {
                        format!("<magenta>{}'s dms</>", ctx.author().tag())
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
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(PREFIX.to_string()),
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_presence(
                    Some(Activity::listening("you, cutie <3")),
                    OnlineStatus::DoNotDisturb,
                )
                .await;
                success!("<bold>{}</> is <green>connected!</>", ready.user.name);
                Ok(Data {})
            })
        });

    framework.run().await?;

    Ok(())
}
