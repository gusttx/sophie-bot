use poise::command;

use crate::{types::{Context, UnitResult}, utils::discord::reply::Reply};

/// Veja a mensagem global
#[command(
    slash_command,
    user_cooldown = 10
)]
pub async fn get(ctx: Context<'_>) -> UnitResult {
    let data = ctx.data();

    let result = format!("**MENSAGEM GLOBAL**\n\n{}", data.message.read().await);
    Reply::with_content(result).send_ok(&ctx).await
}
