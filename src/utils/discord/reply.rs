use super::embed::Embed;
use crate::types::{Context, UnitResult};
use poise::{
    serenity_prelude::{
        ComponentInteraction, CreateActionRow, CreateInteractionResponseFollowup, EditMessage,
        Message, Result as SerenityResult,
    },
    CreateReply, ReplyHandle,
};

#[derive(Default)]
pub struct Reply {
    ephemeral: bool,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    components: Option<Vec<CreateActionRow>>,
}

impl Into<CreateInteractionResponseFollowup> for Reply {
    fn into(self) -> CreateInteractionResponseFollowup {
        let mut followup = CreateInteractionResponseFollowup::default();
        if self.ephemeral {
            followup = followup.ephemeral(true);
        }
        if let Some(content) = self.content {
            followup = followup.content(content);
        }
        if let Some(components) = self.components {
            followup = followup.components(components);
        }
        if let Some(embeds) = self.embeds {
            followup = followup.embeds(embeds.into_iter().map(|e| e.into()).collect());
        }

        followup
    }
}

impl Into<CreateReply> for Reply {
    fn into(self) -> CreateReply {
        let mut reply = CreateReply::default();
        if self.ephemeral {
            reply = reply.ephemeral(true);
        }
        if let Some(content) = self.content {
            reply = reply.content(content);
        }
        if let Some(components) = self.components {
            reply = reply.components(components);
        }

        // reply doesn't support .embeds() yet
        if let Some(embeds) = self.embeds {
            for embed in embeds {
                reply = reply.embed(embed.into());
            }
        }

        reply
    }
}

impl Into<EditMessage> for Reply {
    fn into(self) -> EditMessage {
        let mut edit = EditMessage::new();
        if let Some(content) = self.content {
            edit = edit.content(content);
        }
        if let Some(components) = self.components {
            edit = edit.components(components);
        }
        if let Some(embeds) = self.embeds {
            edit = edit.embeds(embeds.into_iter().map(|e| e.into()).collect());
        }

        edit
    }
}

impl Reply {
    pub fn with_content(content: impl Into<String>) -> Self {
        Self {
            ephemeral: false,
            content: Some(content.into()),
            embeds: None,
            components: None,
        }
    }

    pub fn ephemeral(content: impl Into<String>) -> Self {
        Self {
            ephemeral: true,
            content: Some(content.into()),
            embeds: None,
            components: None,
        }
    }

    pub fn with_embed(embed: Embed) -> Self {
        Self {
            ephemeral: false,
            content: None,
            embeds: Some(vec![embed]),
            components: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            ephemeral: false,
            content: None,
            embeds: Some(Vec::new()),
            components: Some(Vec::new()),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn add_action_row(mut self, row: CreateActionRow) -> Self {
        self.components
            .get_or_insert_with(|| Vec::with_capacity(5))
            .push(row);
        self
    }

    pub fn empty_action_rows(mut self) -> Self {
        self.components = Some(Vec::new());
        self
    }

    pub async fn send<'a, 'b>(self, ctx: &Context<'a>) -> SerenityResult<ReplyHandle<'b>>
    where
        'a: 'b,
    {
        ctx.send(self.into()).await
    }

    pub async fn send_ok(self, ctx: &Context<'_>) -> UnitResult {
        ctx.send(self.into()).await?;
        Ok(())
    }

    pub async fn followup(
        self,
        ctx: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> SerenityResult<Message> {
        interaction.create_followup(ctx, self.into()).await
    }

    pub async fn edit(self, ctx: &Context<'_>, msg: &mut Message) -> SerenityResult<()> {
        msg.edit(ctx, self.into()).await
    }

    pub async fn edit_ok(self, ctx: &Context<'_>, msg: &mut Message) -> UnitResult {
        msg.edit(ctx, self.into()).await?;
        Ok(())
    }

    pub async fn edit_followup(
        self,
        ctx: &Context<'_>,
        interaction: &ComponentInteraction,
        msg: &Message,
    ) -> SerenityResult<Message> {
        interaction.edit_followup(ctx, msg.id, self.into()).await
    }
}
