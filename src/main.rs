use std::time::Duration;

use commands::{misc, moderation, owner};
use once_cell::sync::Lazy;
use poise::{
    serenity_prelude::{self as serenity, Activity, Color, GatewayIntents, OnlineStatus},
    EditTracker, Event, Framework, FrameworkContext, FrameworkError, FrameworkOptions,
    PrefixFrameworkOptions,
};

mod commands;
mod plugins;
mod sources;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub static TOKEN: Lazy<String> = Lazy::new(|| std::env::var("WISP_TOKEN").unwrap());
pub static PREFIX: Lazy<String> = Lazy::new(|| std::env::var("WISP_PREFIX").unwrap());
pub static COLOR: Lazy<Color> = Lazy::new(|| {
    Color::new(u32::from_str_radix(&std::env::var("WISP_COLOR").unwrap(), 16).unwrap())
});

pub struct Data;

async fn context_location(ctx: Context<'_>) -> String {
    if let Some(guild) = ctx.guild() {
        if let Some(channel) = ctx.channel_id().name(&ctx).await {
            format!("#{}, {} ({})", channel, guild.name, guild.id)
        } else {
            format!("{} ({})", guild.name, guild.id)
        }
    } else {
        format!("{}'s dms", ctx.author().tag())
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
    twink::purr!(
        "executed <cyan>{}</> by <bold>{}</> (<italic>{}</>) in <purple>{}</>",
        ctx.command().qualified_name,
        ctx.author().tag(),
        ctx.author().id,
        context_location(ctx).await,
    );
}

pub fn reply_callback(_ctx: Context<'_>, reply: &mut poise::CreateReply<'_>) {
    reply.allowed_mentions(|f| f.replied_user(false));
    reply.reply(true);
}

async fn on_error(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Command { error, ctx } => {
            ctx.send(|r| r.embed(|e| e.description(error.to_string()).color(*COLOR)))
                .await
                .expect("failed to reply with error message");
            twink::hiss!(
                "command <cyan>{}</> returned error <b><red>{:?}</> by <bold>{}</> (<italic>{}</>) in <purple>{}</>",
                ctx.command().name,
                error,
                ctx.author().tag(),
                ctx.author().id,
                context_location(ctx).await,
            );
        }
        FrameworkError::EventHandler { error, event, .. } => {
            twink::hiss!(
                "EventHandler returned error during <cyan>{:?}</> event: <red>{:?}</>",
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                twink::hiss!("Error while handling error: <red>{}</>", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let framework = Framework::builder()
        .token(TOKEN.clone())
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
                misc::avatar::avatar_ctx(),
                misc::spotify::spotify(),
                misc::spotify::spotify_ctx(),
                misc::cute::cute(),
                misc::faye::faye(),
                misc::user::user(),
                misc::user::user_ctx(),
                misc::webm::webm(),
                misc::xkcd::xkcd(),
                moderation::clear::clear(),
                owner::echo::echo(),
                owner::ptolemaea::ptolemaea(),
                owner::register::register(),
            ],
            on_error: |err| Box::pin(on_error(err)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            reply_callback: Some(reply_callback),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(PREFIX.clone()),
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
                    Some(Activity::listening("you, cutie ♡")),
                    OnlineStatus::DoNotDisturb,
                )
                .await;
                twink::purr!("logged in as <bold><purple>@{}</>", ready.user.tag());
                Ok(Data)
            })
        });

    framework.run().await?;

    Ok(())
}
