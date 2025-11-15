mod buttons;
mod select_menu;

pub use buttons::*;
use poise::serenity_prelude::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
pub use select_menu::*;

#[derive(Clone)]
pub enum ActionRow {
    Buttons(ButtonsRow),
    StringSelectMenu(StringSelectMenu),
}

impl Into<CreateActionRow> for ActionRow {
    fn into(self) -> CreateActionRow {
        match self {
            Self::Buttons(row) => CreateActionRow::Buttons(row.buttons),
            Self::StringSelectMenu(row) => CreateActionRow::SelectMenu(CreateSelectMenu::new(
                row.custom_id,
                CreateSelectMenuKind::String {
                    options: row
                        .options
                        .into_iter()
                        .map(|opt| CreateSelectMenuOption::new(opt.0, opt.1))
                        .collect(),
                },
            )),
        }
    }
}