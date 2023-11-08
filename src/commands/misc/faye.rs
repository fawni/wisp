use faye::prelude::{Context as FayeContext, Parser};

use crate::{commands::CommandError, Context, Error};

/// Eval faye
#[poise::command(prefix_command, slash_command)]
pub async fn faye(
    ctx: Context<'_>,
    #[description = "Expression to eval"] expression: String,
) -> Result<(), Error> {
    let mut faye_ctx = FayeContext::default();

    let parser = Parser::new(&expression);

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

    ctx.send(|r| {
        r.content(format!(
            "```clj\n{}\n```",
            res.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        ))
    })
    .await?;

    Ok(())
}
