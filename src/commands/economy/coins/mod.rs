mod add;
mod check;
mod remove;
mod reset;
mod send;
mod set;

use crate::types::{Context, UnitResult};
use poise::command;

pub use super::DEPARTMENT_NAME;
const EMBED_COLOR: u32 = 0xFFCC4D;

#[command(
    slash_command,
    prefix_command,
    subcommands(
        "check::check",
        "send::send",
        "add::add",
        "set::set",
        "remove::remove",
        "reset::reset"
    ),
    subcommand_required
)]
pub async fn coins(_: Context<'_>) -> UnitResult {
    Ok(())
}
