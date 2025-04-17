use log::info;
use poise::command;
use crate::{types::{Context, UnitResult}, utils::discord::reply::Reply};

/// Altere a mensagem global
#[command(
    slash_command,
    user_cooldown = 60
)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Nova mensagem"]
    #[max_length = 400]
    message: String
) -> UnitResult {
    let data = ctx.data();
    let author = ctx.author();

    info!(
        "{}/{} has set the global message to \"{}\"",
        author.name, author.id, message
    );
    
    let result = format!("**NOVA MENSAGEM GLOBAL**\n\n{}", message);
    *data.message.write().await = message;
    Reply::ephemeral(result).send_ok(&ctx).await
}