mod avatar;

use poise::command;
use crate::types::{Context, UnitResult};

pub use super::DEPARTMENT_NAME;
pub use avatar::avatar_context;

#[command(
    slash_command,
    subcommands("avatar::avatar"),
    subcommand_required
)]
pub async fn user(_: Context<'_>) -> UnitResult {
    Ok(())
}
