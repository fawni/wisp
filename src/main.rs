use std::time::Duration;

use lazy_static::lazy_static;
use paris::{info, success};
use poise::serenity_prelude::{self as serenity, Activity, OnlineStatus};

mod commands;
mod plugins;
mod sources;

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
        plugins::handle_message(ctx.clone(), msg.clone()).await?;
    }

    Ok(())
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
                commands::misc::help::help(),
                commands::misc::ping::ping(),
                commands::misc::avatar::avatar(),
                commands::owner::echo::echo(),
                commands::moderation::clear::clear(),
                commands::misc::cute::cute(),
                commands::misc::webm::webm(),
                commands::misc::xkcd::xkcd(),
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
