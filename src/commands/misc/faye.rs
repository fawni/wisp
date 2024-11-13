use faye::prelude::{Context as FayeContext, Parser};
use poise::CreateReply;

use crate::{commands::CommandError, Context, Error};

/// Eval a faye expression
#[poise::command(prefix_command, slash_command, track_edits, category = "Miscellaneous")]
pub async fn faye(
    ctx: Context<'_>,
    #[description = "Expression to eval"] expression: String,
) -> Result<(), Error> {
    let mut faye_ctx = FayeContext::default();

    let mut parser = Parser::new(&expression);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => return Err(CommandError::FayeParserError(err).into()),
    };

    let mut res = vec![];

    for node in ast {
        match faye_ctx.eval(&node) {
            Ok(expr) => res.push(expr),
            Err(err) => return Err(CommandError::FayeError(err).into()),
        };
    }

    ctx.send(CreateReply::default().content(format!(
            "```clj\n{}\n```",
            res.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )))
    .await?;

    Ok(())
}
