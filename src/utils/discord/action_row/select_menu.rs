use poise::serenity_prelude::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};

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

impl Into<CreateActionRow> for StringSelectMenu {
    fn into(self) -> CreateActionRow {
        CreateActionRow::SelectMenu(CreateSelectMenu::new(
            self.custom_id,
            CreateSelectMenuKind::String {
                options: self
                    .options
                    .into_iter()
                    .map(|opt| CreateSelectMenuOption::new(opt.0, opt.1))
                    .collect(),
            },
        ))
    }
}
