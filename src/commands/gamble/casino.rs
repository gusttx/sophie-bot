use super::blackjack::blackjack;
use super::info::info;

use crate::types::{Context, UnitResult};
use poise::command;

#[command(slash_command, subcommands("blackjack", "info"))]
pub async fn casino(_: Context<'_>) -> UnitResult {
    Ok(())
}
