use super::{DEPARTMENT_NAME, EMBED_COLOR};
use crate::{
    database::User,
    types::{Context, UnitResult},
    utils::discord::{embed::Embed, reply::Reply},
};
use poise::{command, serenity_prelude::User as SerenityUser};

/// Envie moedas para outro usuário
#[command(slash_command, user_cooldown = 10)]
pub async fn send(
    ctx: Context<'_>,
    #[description = "Usuário para visualizar"] user: SerenityUser,
    #[description = "Quantia de moedas para enviar"]
    #[min = 1]
    qnt: u32,
    #[description = "Descrição do envio"]
    #[max_length = 200]
    description: Option<String>,
) -> UnitResult {
    let author = ctx.author();
    let db = &ctx.data().database;

    if user.id == author.id {
        return Reply::ephemeral(
            ":rage: Proibido! Você está P-R-O-I-B-I-D-O de enviar moedas para você mesmo",
        )
        .send_ok(&ctx)
        .await;
    } else if user.bot {
        return Reply::ephemeral(":rofl: O cara quer enviar moedas para bot, hahahahaha")
            .send_ok(&ctx)
            .await;
    }

    let mut sender = User::get_or_create(db, author.id).await?;
    if sender.coins < qnt {
        return Reply::ephemeral(
            ":face_with_raised_eyebrow: Vai tirar essas moedas de onde irmão? tu nem isso",
        )
        .send_ok(&ctx)
        .await;
    }
    sender.send_coins(db, user.id, qnt).await?;

    let inflection = if qnt > 1 { "moedas" } else { "moeda" };
    let embed = Embed::new(EMBED_COLOR, ":coin: Envio de Moedas")
        .inline_field("Remetente", author.to_string())
        .inline_field("Destinatário", user.to_string())
        .inline_field("Quantia", format!("{qnt} {inflection}"))
        .optional_field("Descrição", description)
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed).send_ok(&ctx).await
}
