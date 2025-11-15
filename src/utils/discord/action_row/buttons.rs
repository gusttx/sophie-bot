use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton, ReactionType};

use crate::utils::discord::action_row::ActionRow;

pub struct Button {
    id: String,
    name: String,
    emoji: Option<ReactionType>,
}

impl Button {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            emoji: None,
        }
    }

    pub fn with_emoji(
        id: impl Into<String>,
        name: impl Into<String>,
        emoji: impl Into<ReactionType>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            emoji: Some(emoji.into()),
        }
    }
}

#[derive(Clone)]
pub struct ButtonsRow {
    pub buttons: Vec<CreateButton>,
    pub link_buttons: Vec<usize>
}

impl Into<ActionRow> for ButtonsRow {
    fn into(self) -> ActionRow {
        ActionRow::Buttons(self)
    }
}

impl ButtonsRow {
    pub fn new() -> Self {
        Self {
            buttons: Vec::with_capacity(5),
            link_buttons: Vec::with_capacity(5),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty()
    }

    fn insert_button(mut self, style: ButtonStyle, button: Button) -> Self {
        let mut create_button = CreateButton::new(button.id)
            .label(button.name)
            .style(style);

        if let Some(emoji) = button.emoji {
            create_button = create_button.emoji(emoji);
        }

        self.buttons.push(create_button);
        self
    }

    #[allow(dead_code)]
    pub fn add_blurple(self, button: Button) -> Self {
        self.insert_button(ButtonStyle::Primary, button)
    }

    pub fn add_green(self, button: Button) -> Self {
        self.insert_button(ButtonStyle::Success, button)
    }

    pub fn add_grey(self, button: Button) -> Self {
        self.insert_button(ButtonStyle::Secondary, button)
    }

    pub fn add_red(self, button: Button) -> Self {
        self.insert_button(ButtonStyle::Danger, button)
    }

    pub fn add_link(mut self, url: impl Into<String>, label: impl Into<String>) -> Self {
        let create_button = CreateButton::new_link(url)
            .label(label);

        self.link_buttons.push(self.buttons.len());
        self.buttons.push(create_button);

        self
    }

    pub fn retain_links(mut self) -> Self {
        let filtered_buttons: Vec<CreateButton> = self
            .link_buttons
            .iter()
            .filter_map(|&i| self.buttons.get(i).cloned())
            .collect();

        self.buttons = filtered_buttons;
        self.link_buttons = (0..self.buttons.len()).collect();

        self
    }
}