use crate::database::User;
use crate::types::{BlackjackGame, BlackjackResult, Context, InvocationData, UnitResult};
use crate::utils::discord::action_row::{Button, ButtonsRow};
use crate::utils::discord::embed::Embed;
use crate::utils::discord::reply::Reply;
use futures::StreamExt;
use poise::serenity_prelude::ComponentInteractionCollector;
use poise::{command, serenity_prelude::User as SerenityUser};

use super::DEPARTMENT_NAME;
const EMBED_COLOR: u32 = 0xFF3344;

/// Jogue uma partida de Blackjack (21) com o bot
#[command(slash_command, user_cooldown = 10)]
pub async fn blackjack(
    ctx: Context<'_>,
    #[description = "Sua aposta"]
    #[min = 1]
    #[max = 100_000]
    bet: u32,
) -> UnitResult {
    let author = ctx.author();
    let db = &ctx.data().database;
    let config = &ctx.data().config;

    let mut db_user = User::get_or_create(db, author.id).await?;
    if db_user.coins < bet {
        return Reply::ephemeral(":black_bird: Você não tem moedas suficientes")
            .send_ok(&ctx)
            .await;
    }

    let mut game = BlackjackGame::new(config.blackjack.decks.get());
    let embed = create_result_embed(&game, author, bet, false);

    if game.is_finished() {
        db_user.coins += bet;
        db_user.update(&db).await?;

        return Reply::with_embed(embed).send_ok(&ctx).await;
    }

    db_user.coins -= bet;
    db_user.update(&db).await?;

    InvocationData::refound(&ctx, author.id, bet).await;

    let buttons = ButtonsRow::new()
        .add_green(Button::new("hit", "Pedir Carta"))
        .add_red(Button::new("stop", "Parar"));

    let mut message = Reply::with_embed(embed)
        .add_action_row(buttons.into())
        .send(&ctx)
        .await?
        .into_message()
        .await?;

    InvocationData::edit_message(&ctx, message.clone()).await;

    let mut collector = ComponentInteractionCollector::new(ctx)
        .message_id(message.id)
        .timeout(config.timeout.blackjack)
        .stream();

    while let Some(interaction) = collector.next().await {
        if interaction.user.id != author.id {
            Reply::ephemeral(":rage: Esse jogo não é teu piá")
                .followup(&ctx, &interaction)
                .await?;
            continue;
        }

        match interaction.data.custom_id.as_str() {
            "hit" => game.take_card(),
            _ => game.dealer_turn(),
        }

        if game.is_finished() {
            break;
        }

        let embed = create_result_embed(&game, author, bet, false);
        Reply::with_embed(embed).edit(&ctx, &mut message).await?;
    }

    if game.is_finished() {
        match game.result {
            BlackjackResult::Win => {
                db_user.coins += bet * 2;
                db_user.update(&db).await?;
            }
            BlackjackResult::Tie => {
                db_user.coins += bet;
                db_user.update(&db).await?;
            }
            _ => {}
        }

        InvocationData::clear_refound(&ctx).await;
    }

    let embed = create_result_embed(&game, author, bet, true);
    Reply::with_embed(embed)
        .empty_action_rows()
        .edit_ok(&ctx, &mut message)
        .await
}

fn create_result_embed(
    game: &BlackjackGame,
    author: &SerenityUser,
    bet: u32,
    timeout: bool,
) -> Embed {
    let players_field = game.player_hand.get_field();
    let dealers_field = game.dealer_hand.get_field();

    let embed = Embed::new(EMBED_COLOR, ":joystick: Blackjack")
        .inline_field("Mão do Jogador", &players_field)
        .inline_field("Mão do Dealer", &dealers_field)
        .field("", "O dealer para com um total maior ou igual a 17")
        .footer(DEPARTMENT_NAME);

    let default_desc = format!("**Apostador:** {author}\n**Aposta:** {bet} :coin:");

    if game.is_finished() {
        let result = match game.result {
            BlackjackResult::Win => ":partying_face: **O jogador venceu!**",
            BlackjackResult::Tie => ":neutral_face: **O jogador empatou!**",
            _ => ":pleading_face: **O jogador perdeu**",
        };

        return embed.desc(format!("{result}\n\n{default_desc}"));
    }

    let desc = if timeout {
        format!(":pleading_face: **O jogador desistiu**\n\n{default_desc}")
    } else {
        default_desc
    };

    embed.desc(desc)
}
