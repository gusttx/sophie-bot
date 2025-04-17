use crate::{
    types::{Context, UnitResult},
    utils::discord::{embed::Embed, reply::Reply},
};
use poise::command;

/// Leia uma informação muito importante sobre apostas
#[command(
    slash_command,
    user_cooldown = 20
)]
pub async fn info(ctx: Context<'_>) -> UnitResult {
    let embed = Embed::new(0xA8DEF2, "Não desista!")
        .desc(
            "Você sabia que **90%** dos apostadores __desistem__ pouco antes de **lucrar muito**?\n
             E você **não** quer fazer parte desse __grupo__, né? A chave para o sucesso nas apostas está na **persistência**.\n
             **Lembre-se**, a sorte favorece os **ousados**. Não deixe que o __medo de perder__ o impeça de **ganhar**!"
        )
        .footer(super::DEPARTMENT_NAME)
        .large_image("https://i.imgur.com/5OspQZI.png");

    Reply::with_embed(embed).send_ok(&ctx).await
}
