use crate::{
    database::{Db, User}, types::{JankenponChoice, JankenponResult}, utils::discord::{
        action_row::{Button, ButtonsRow},
        embed::Embed,
        reply::Reply,
    }
};
use poise::serenity_prelude::{User as SerenityUser, UserId};
use thiserror::Error;

use super::DEPARTMENT_NAME;
const EMBED_COLOR: u32 = 0xCE3E32;

#[derive(Error, Debug)]
pub enum PrepareError {
    #[error("User '{0}' not found in database")]
    UserNotFound(UserId),
    #[error("User does not have enough coins")]
    NotEnoughCoins,
    #[error("Failed to update user '{0}' in database")]
    FailToUpdate(UserId),
    #[error("No bet specified")]
    NoBet,
}

pub async fn prepare_player(
    db: &Db,
    user_id: UserId,
    bet: Option<u32>,
) -> Result<User, PrepareError> {
    let bet = match bet {
        Some(bet) => bet,
        None => return Err(PrepareError::NoBet),
    };

    let mut user = User::get_or_create(db, user_id)
        .await
        .map_err(|_| PrepareError::UserNotFound(user_id))?;

    if user.coins < bet {
        return Err(PrepareError::NotEnoughCoins);
    }
    user.coins -= bet;

    user.update(db)
        .await
        .map_err(|_| PrepareError::FailToUpdate(user_id))?;
    Ok(user)
}

pub fn create_challenge_reply(
    author: &SerenityUser,
    opponent: &SerenityUser,
    bet: Option<u32>,
) -> Reply {
    let embed = Embed::new(
        EMBED_COLOR,
        ":index_pointing_at_the_viewer: Duelo de Jankenpon",
    )
    .desc(format!(
        "{} desafiou {} para um duelo de jankenpon",
        author.name, opponent.name
    ))
    .inline_field("Desafiante", author.to_string())
    .inline_field("Desafiado", opponent.to_string())
    .optional_inline_field("Aposta", bet.map(|bet| format!("{} :coin:", bet)))
    .footer(DEPARTMENT_NAME);

    let buttons = ButtonsRow::new()
        .add_grey(Button::with_emoji("rock", "Pedra", '✊'))
        .add_grey(Button::with_emoji("paper", "Papel", '🖐'))
        .add_grey(Button::with_emoji("scissors", "Tesoura", '✌'));

    Reply::with_embed(embed).add_action_row(buttons.into())
}

pub fn create_result_reply(
    challenger: &SerenityUser,
    challenger_choice: JankenponChoice,
    opponent: &SerenityUser,
    opponent_choice: JankenponChoice,
    bet: Option<u32>,
) -> Reply {
    let result = challenger_choice.compare(opponent_choice);

    let (result_text, challenger_title, opponent_title) = match result {
        JankenponResult::Win => ("**venceu**", "Desafiante :crown:", "Desafiado"),
        JankenponResult::Tie => ("**empatou** com", "Desafiante", "Desafiado"),
        JankenponResult::Lose => ("**perdeu** de", "Desafiante", ":crown: Desafiado"),
    };

    let description = format!(
        "{} escolhendo __{}__, {result_text} {} que escolheu __{}__!",
        challenger.name,
        challenger_choice.get_name(),
        opponent.name,
        opponent_choice.get_name(),
    );

    let embed = Embed::new(EMBED_COLOR, ":partying_face: Resultado Jankenpon")
        .desc(description)
        .inline_field(
            challenger_title,
            format!("{} {}", challenger.name, challenger_choice.get_emoji()),
        )
        .inline_field(
            opponent_title,
            format!("{} {}", opponent_choice.get_emoji(), opponent.name),
        )
        .optional_inline_field("Prêmio", bet.map(|bet| format!("{} :coin:", bet * 2)))
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed)
}