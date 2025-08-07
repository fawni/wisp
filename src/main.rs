use std::time::Duration;

use commands::{misc, moderation, owner};
use once_cell::sync::Lazy;
use poise::{
    serenity_prelude::{
        self as serenity, ActivityData, ClientBuilder, Color, CreateAllowedMentions, CreateEmbed,
        FullEvent, GatewayIntents, OnlineStatus,
    },
    CreateReply, EditTracker, Framework, FrameworkContext, FrameworkError, FrameworkOptions,
    PrefixFrameworkOptions,
};
use sources::gemini::GeminiClient;

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

pub static WOLFRAM: Lazy<String> = Lazy::new(|| std::env::var("WOLFRAM_APP_ID").unwrap());
pub static GEMINI_KEY: Lazy<String> = Lazy::new(|| std::env::var("GEMINI_API_KEY").unwrap());
pub static GEMINI_MODEL: Lazy<String> = Lazy::new(|| std::env::var("GEMINI_MODEL").unwrap());
pub static GEMINI_PROMPT: Lazy<String> =
    Lazy::new(|| std::env::var("GEMINI_SYSTEM_PROMPT").unwrap());

pub struct Data {
    gemini_client: GeminiClient,
}

async fn context_location(ctx: Context<'_>) -> String {
    if let Some(guild) = ctx.partial_guild().await {
        if let Ok(channel) = ctx.channel_id().name(&ctx.http()).await {
            format!("#{}, {} ({})", channel, guild.name, guild.id)
        } else {
            format!("{} ({})", guild.name, guild.id)
        }
    } else {
        format!("{}'s dms", ctx.author().tag())
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    if let FullEvent::Message { new_message: msg } = event {
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

pub fn reply_callback(_ctx: Context<'_>, reply: CreateReply) -> CreateReply {
    reply
        .allowed_mentions(CreateAllowedMentions::default().replied_user(false))
        .reply(true)
}

async fn on_error(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Command { error, ctx, .. } => {
            // ctx.send(|r| r.embed(|e| e.description(error.to_string()).color(*COLOR)))
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .description(error.to_string())
                        .color(*COLOR),
                ),
            )
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
                event.snake_case_name(),
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
    let token = TOKEN.clone();
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES;
    let framework = Framework::builder()
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
                misc::gemini::wisp(),
                misc::gemini::generate(),
                misc::tts::tts(),
                misc::wolfram::wolfram(),
                misc::user::user(),
                misc::user::user_info_ctx(),
                misc::role::role(),
                misc::webm::webm(),
                misc::xkcd::xkcd(),
                moderation::clear::clear(),
                owner::echo::echo(),
                // owner::sys::sys(),
                owner::register::register(),
            ],
            on_error: |err| Box::pin(on_error(err)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            reply_callback: Some(reply_callback),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(PREFIX.clone()),
                edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(3600)).into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, user_data| {
                Box::pin(event_handler(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_presence(Some(ActivityData::listening("♡")), OnlineStatus::Online);
                twink::purr!("logged in as <bold><purple>@{}</>", ready.user.tag());

                let gemini_client = GeminiClient::new(
                    GEMINI_KEY.clone(),
                    None,
                    Some(GEMINI_PROMPT.clone()),
                    "wisp".to_owned(),
                );
                let data = Data { gemini_client };
                Ok(data)
            })
        })
        .build();
    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
