mod data;
mod jankenpon;
mod economy;
mod blackjack;
mod weather;
mod invocation_data;
#[cfg(feature = "onlinefix")]
mod onlinefix;
#[cfg(feature = "onlinefix")]
mod torrent;
mod meval_context;

pub use data::Data;
pub use jankenpon::*;
pub use economy::*;
pub use blackjack::*;
pub use meval_context::*;
pub use weather::*;
pub use invocation_data::*;
#[cfg(feature = "onlinefix")]
pub use onlinefix::*;
#[cfg(feature = "onlinefix")]
pub use torrent::*;

use poise::serenity_prelude::{Message, ReactionType};

pub type UnitResult = Result<(), anyhow::Error>;
pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

pub trait ContextUtils {
    fn msg(&self) -> Option<&Message>;
    async fn react(&self, emoji: impl Into<ReactionType>) -> UnitResult;
}

impl ContextUtils for Context<'_> {
    fn msg(&self) -> Option<&Message> {
        match self {
            Context::Prefix(poise::PrefixContext { msg, .. }) => Some(msg),
            _ => None,
        }
    }

    async fn react(&self, emoji: impl Into<ReactionType>) -> UnitResult {
        if let Some(msg) = self.msg() {
            msg.react(self, emoji).await?;
        }
        Ok(())
    }
}