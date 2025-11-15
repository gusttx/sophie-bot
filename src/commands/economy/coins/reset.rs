use crate::{
    database::User,
    types::{Context, ContextUtils, UnitResult},
};
use poise::{
    command,
    serenity_prelude::{ReactionCollector, ReactionType, User as SerenityUser},
};

#[command(prefix_command, owners_only)]
pub async fn reset(ctx: Context<'_>, user: SerenityUser) -> UnitResult {
    let data = ctx.data();

    if user.bot {
        return ctx.react('👎').await;
    }

    User::set_coins(&data.database, user.id, data.config.economy.initial_coins).await?;
    _ = ctx.react('👊').await;

    let Some(message) = ctx.msg() else {
        return Ok(());
    };

    let reaction = ReactionCollector::new(ctx)
        .author_id(ctx.author().id)
        .message_id(message.id)
        .filter(|reaction| reaction.emoji == ReactionType::Unicode('👊'.to_string()))
        .timeout(ctx.data().config.timeout.owner_response)
        .await;

    if reaction.is_some() {
        _ = message.delete(&ctx).await;
    }

    Ok(())
}
