use poise::serenity_prelude::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};

use crate::utils::discord::action_row::ActionRow;

#[derive(Clone)]
pub struct StringSelectMenu {
    pub custom_id: String,
    pub options: Vec<(String, String)>,
}

impl StringSelectMenu {
    pub fn new(custom_id: impl Into<String>, initial_capacity: usize) -> Self {
        Self {
            custom_id: custom_id.into(),
            options: Vec::with_capacity(initial_capacity),
        }
    }

    pub fn add_option(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.options.push((label.into(), value.into()));
    }
}

impl Into<ActionRow> for StringSelectMenu {
    fn into(self) -> ActionRow {
        ActionRow::StringSelectMenu(self)
    }
}
