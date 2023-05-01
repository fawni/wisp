use std::time::Duration;

use commands::{misc, moderation, owner};
use paris::{error, info, success};
use poise::{
    serenity_prelude::{self as serenity, Activity, GatewayIntents, OnlineStatus},
    EditTracker, Event, Framework, FrameworkContext, FrameworkError, FrameworkOptions,
    PrefixFrameworkOptions,
};

mod commands;
mod plugins;
mod sources;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

async fn context_location(ctx: Context<'_>) -> String {
    if let Some(guild) = ctx.guild() {
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
    }
}

async fn event_listener(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    if let Event::Message { new_message: msg } = event {
        plugins::handle_message(ctx.clone(), msg.clone()).await?;
    }

    Ok(())
}

async fn post_command(ctx: Context<'_>) {
    info!(
        "executed <green>{}</> by <bold>{}</> (<italic>{}</>) in {}",
        ctx.command().qualified_name,
        ctx.author().tag(),
        ctx.author().id,
        context_location(ctx).await,
    );
}

async fn on_error(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Command { error, ctx } => {
            ctx.send(|r| r.embed(|e| e.description(error.to_string()).color(0xE83F80)))
                .await
                .expect("failed to reply with error message");
            error!(
                "command <green>{}</> returned error <bold><red>{:?}</> by <bold>{}</> (<italic>{}</>) in {}",
                ctx.command().name,
                error,
                ctx.author().tag(),
                ctx.author().id,
                context_location(ctx).await,
            );
        }
        FrameworkError::EventHandler { error, event, .. } => {
            error!(
                "EventHandler returned error during <green>{:?}</> event: <red>{:?}</>",
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: <red>{}</>", e)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let framework = Framework::builder()
        .token(std::env::var("WISP_TOKEN").expect("missing WISP_TOKEN env var"))
        .intents(
            GatewayIntents::non_privileged()
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_PRESENCES,
        )
        .options(FrameworkOptions {
            commands: vec![
                misc::help::help(),
                misc::ping::ping(),
                misc::avatar::avatar(),
                misc::spotify::spotify(),
                misc::cute::cute(),
                misc::webm::webm(),
                misc::xkcd::xkcd(),
                moderation::clear::clear(),
                owner::echo::echo(),
            ],
            on_error: |err| Box::pin(on_error(err)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(std::env::var("WISP_PREFIX")?),
                edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(3600))),
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
