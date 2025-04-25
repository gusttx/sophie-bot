use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton, ReactionType};

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

pub struct ButtonsRow {
    buttons: Vec<CreateButton>,
}

impl Into<CreateActionRow> for ButtonsRow {
    fn into(self) -> CreateActionRow {
        CreateActionRow::Buttons(self.buttons)
    }
}

impl ButtonsRow {
    pub fn new() -> Self {
        Self {
            buttons: Vec::with_capacity(5),
        }
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

    #[allow(dead_code)]
    pub fn add_link(mut self, url: impl Into<String>, label: impl Into<String>) -> Self {
        let create_button = CreateButton::new_link(url)
            .label(label);

        self.buttons.push(create_button);
        self
    }
}