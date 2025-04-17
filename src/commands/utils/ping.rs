use super::DEPARTMENT_NAME;
use crate::{types::{Context, UnitResult}, utils::discord::{embed::Embed, reply::Reply}};
use poise::command;

/// Pong!
#[command(
    slash_command,
    user_cooldown = 10
)]
pub async fn ping(ctx: Context<'_>) -> UnitResult {
    let result = ctx.ping().await.as_millis();
    let embed = Embed::new(0x63B5D0, ":ping_pong: Pong!")
        .desc(format!("**Heartbeat Latency:** ``{}ms``", result))
        .footer(DEPARTMENT_NAME);
    
    Reply::with_embed(embed).send_ok(&ctx).await
}
