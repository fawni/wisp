pub mod misc;
pub mod moderation;
pub mod owner;

#[derive(thiserror::Error, Debug)]
enum CommandError {
    #[error("This command can only be used in a guild")]
    GuildOnly,

    #[error("User currently has no presence/activity shared in this guild")]
    PresenceNotFound,

    #[error("User is not currently listening to Spotify")]
    NoSpotify,

    #[error("Faye parser error: {0}")]
    FayeParserError(#[from] faye::prelude::ParserError),

    #[error("Faye eval error: {0}")]
    FayeError(#[from] faye::prelude::EvalError),
}
