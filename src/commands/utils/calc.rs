use poise::{command, serenity_prelude::AutocompleteChoice};
use crate::{types::{Context, UnitResult}, utils::discord::{embed::Embed, reply::Reply}};

use super::DEPARTMENT_NAME;

async fn autocomplete_calc(
    ctx: Context<'_>,
    expression: &str
) -> [AutocompleteChoice; 1] {
    let result = match ctx.data().meval_context.eval(expression.to_lowercase()) {
        Ok(result) => result.to_string(),
        Err(_) => "Invalid expression".to_string(),
    };

    [AutocompleteChoice::new(result, expression)]
}

/// Calculate math expressions
#[command(
    slash_command,
    user_cooldown = 10
)]
pub async fn calc(
    ctx: Context<'_>,
    #[autocomplete = autocomplete_calc]
    #[description = "Math expression to calculate"]
    expression: String,
) -> UnitResult {
    let expression = expression.to_lowercase();

    let result = match ctx.data().meval_context.eval(&expression) {
        Ok(result) => result.to_string(),
        Err(_) => "Invalid expression".to_string(),
    };

    let embed = Embed::new(0x63B5D0, format!("{}'s calculation", ctx.author().name))
        .desc(format!("```rs\n{} = {}```", expression, result))
        .small_image("https://media.discordapp.net/attachments/1351999615926403123/1351999699774734356/calculator-2374442_1280.png")
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed).send_ok(&ctx).await
}
