mod utils;
use futures::StreamExt;
use utils::*;

pub use super::DEPARTMENT_NAME;
use crate::{
    database::User,
    types::{Context, InvocationData, JankenponChoice, JankenponResult, UnitResult},
    utils::discord::reply::Reply,
};
use anyhow::bail;
use poise::{
    command,
    serenity_prelude::{ComponentInteractionCollector, User as SerenityUser},
};

/// Desafie um jogador ou bot para um duelo de Jankenpon
#[command(slash_command, user_cooldown = 10)]
pub async fn jankenpon(
    ctx: Context<'_>,
    #[description = "Seu oponente"] opponent: SerenityUser,
    #[description = "Sua escolha"] choice: JankenponChoice,
    #[description = "Aposta"]
    #[min = 1]
    #[max = 100_000]
    bet: Option<u32>,
) -> UnitResult {
    let author = ctx.author();
    let db = &ctx.data().database;
    let config = &ctx.data().config;

    if author.id == opponent.id {
        return Reply::ephemeral(":pensive: Você precisa fazer mais amigos")
            .send_ok(&ctx)
            .await;
    } else if opponent.bot && bet.is_some() {
        return Reply::ephemeral(":chipmunk: Bots não podem apostar")
            .send_ok(&ctx)
            .await;
    } else if opponent.bot {
        return create_result_reply(author, choice, &opponent, JankenponChoice::random(), None)
            .send_ok(&ctx)
            .await;
    }

    let user = match prepare_player(db, author.id, bet).await {
        Ok(user) => {
            InvocationData::refound(&ctx, author.id, bet.unwrap()).await;
            Some(user)
        }
        Err(PrepareError::NoBet) => None,
        Err(PrepareError::NotEnoughCoins) => {
            return Reply::ephemeral(":pensive: Você não tem moedas suficientes")
                .send_ok(&ctx)
                .await;
        }
        Err(err) => bail!(err),
    };

    let mut message = create_challenge_reply(author, &opponent, bet)
        .send(&ctx)
        .await?
        .into_message()
        .await?;

    InvocationData::edit_message(&ctx, message.clone()).await;

    let mut collector = ComponentInteractionCollector::new(ctx)
        .message_id(message.id)
        .timeout(config.timeout.jankenpon)
        .stream();

    while let Some(interaction) = collector.next().await {
        if interaction.user.id == author.id {
            Reply::ephemeral(":raised_hand::raised_back_of_hand: Calma calabreso")
                .followup(&ctx, &interaction)
                .await?;
            continue;
        }
        if interaction.user.id != opponent.id {
            Reply::ephemeral(":ghost: Você não foi convidado amigão")
                .followup(&ctx, &interaction)
                .await?;
            continue;
        }

        let opponent_choice = JankenponChoice::parse(&interaction.data.custom_id);

        let opponent_user = match prepare_player(db, opponent.id, bet).await {
            Ok(user) => {
                InvocationData::refound(&ctx, opponent.id, bet.unwrap()).await;
                Some(user)
            }
            Err(PrepareError::NoBet) => None,
            Err(PrepareError::NotEnoughCoins) => {
                Reply::ephemeral(":pensive: Você não tem moedas suficientes")
                    .followup(&ctx, &interaction)
                    .await?;
                continue;
            }
            Err(err) => bail!(err),
        };

        if let (Some(mut user), Some(mut opponent_user)) = (user, opponent_user) {
            let prize = bet.unwrap() * 2;
            match opponent_choice.compare(choice) {
                JankenponResult::Win => opponent_user.coins += prize,
                JankenponResult::Lose => user.coins += prize,
                JankenponResult::Tie => {
                    user.coins += prize / 2;
                    opponent_user.coins += prize / 2;
                }
            }

            User::transaction_update(db, vec![user, opponent_user]).await?;
            InvocationData::clear_refound(&ctx).await;
        }

        return create_result_reply(author, choice, &opponent, opponent_choice, bet)
            .empty_action_rows()
            .edit_ok(&ctx, &mut message)
            .await;
    }

    if let Some(mut user) = user {
        user.coins += bet.unwrap();
        user.update(db).await?;
        InvocationData::clear_refound(&ctx).await;
    }

    Reply::default()
        .empty_action_rows()
        .edit_ok(&ctx, &mut message)
        .await
}
