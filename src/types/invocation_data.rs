use std::ops::DerefMut;
use poise::serenity_prelude::{Message, UserId};

use super::Context;

#[derive(Default)]
pub struct InvocationData {
    pub message_to_edit: Option<Message>,
    pub coins_to_refound: Vec<(UserId, u32)>,
}

impl InvocationData {
    async fn from_ctx<'a>(ctx: &'a Context<'_>) -> impl DerefMut<Target = InvocationData> + 'a {
        if let Some(data) = ctx.invocation_data::<InvocationData>().await {
            return data;
        }

        let data = InvocationData::default();
        ctx.set_invocation_data(data).await;
        ctx.invocation_data().await.unwrap()
    }

    pub async fn edit_message(ctx: &Context<'_>, msg: Message) {
        let mut data = Self::from_ctx(ctx).await;
        data.message_to_edit = Some(msg);
    }

    pub async fn refound(ctx: &Context<'_>, user_id: UserId, coins: u32) {
        let mut data = Self::from_ctx(ctx).await;
        data.coins_to_refound.push((user_id, coins));
    }

    pub async fn clear_refound(ctx: &Context<'_>) {
        let mut data = Self::from_ctx(ctx).await;
        data.coins_to_refound.clear();
    }
}