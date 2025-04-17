use poise::command;

use crate::types::{Context, UnitResult};

mod get;
mod set;

#[command(slash_command, subcommands("set::set", "get::get"))]
pub async fn message(_: Context<'_>) -> UnitResult {
    Ok(())
}