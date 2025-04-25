use super::DEPARTMENT_NAME;
use crate::{
    types::{Context, UnitResult},
    utils::discord::{action_row::ButtonsRow, embed::Embed, reply::Reply},
};
use poise::{command, serenity_prelude::User};

fn avatar_reply(user: &User) -> Reply {
    let avatar = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());

    let embed = Embed::new(0x961AFF, format!(":frame_photo: Avatar de {}", user.name))
        .large_image(&avatar)
        .footer(DEPARTMENT_NAME);

    let buttons = ButtonsRow::new().add_link(avatar, "Abrir no navegador");

    Reply::with_embed(embed).add_action_row(buttons.into())
}

/// Displays your, or another user's avatar
#[command(slash_command, user_cooldown = 10)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user that you want to see the avatar"] user: Option<User>,
) -> UnitResult {
    let user = user.as_ref().unwrap_or(ctx.author());
    avatar_reply(user).send_ok(&ctx).await
}

#[command(context_menu_command = "View avatar")]
pub async fn avatar_context(ctx: Context<'_>, user: User) -> UnitResult {
    avatar_reply(&user).send_ok(&ctx).await
}
