use super::blackjack::blackjack;
use super::info::info;

use crate::types::{Context, UnitResult};
use poise::command;

#[command(slash_command, subcommands("blackjack", "info"), subcommand_required)]
pub async fn casino(_: Context<'_>) -> UnitResult {
    Ok(())
}
