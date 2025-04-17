use super::{DEPARTMENT_NAME, EMBED_COLOR};
use crate::database::User;
use crate::types::{CoinsStatus, Context, UnitResult};
use crate::utils::discord::{embed::Embed, reply::Reply};
use poise::{command, serenity_prelude::User as SerenityUser};

/// Veja seu saldo de moedas ou o saldo de outro usuário
#[command(
    slash_command,
    user_cooldown = 10
)]
pub async fn check(
    ctx: Context<'_>,
    #[description = "Usuário para visualizar"] user: Option<SerenityUser>,
) -> UnitResult {
    let target = user.unwrap_or(ctx.author().to_owned());

    if target.bot {
        return Reply::ephemeral(":rofl: O cara quer enviar moedas para bot, hahahahaha")
            .send_ok(&ctx)
            .await;
    }

    let user = User::get_or_create(&ctx.data().database, target.id).await?;

    let desc = format!(
        "{} {} possui {} moedas!",
        CoinsStatus::get(user.coins).get_emoji(),
        target,
        user.coins
    );

    let embed = Embed::new(EMBED_COLOR, ":coin: Saldo de Moedas")
        .desc(desc)
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed).send_ok(&ctx).await
}
