use crate::types::{Context, UnitResult};
use poise::command;

#[command(slash_command)]
pub async fn user(_: Context<'_>) -> UnitResult {
    Ok(())
}
