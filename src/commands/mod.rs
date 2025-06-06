mod fun {
    pub mod jankenpon;
    pub mod message;
    pub mod salve;

    pub const DEPARTMENT_NAME: &str = "© SophieParty";
}

mod utils {
    #[cfg(feature = "onlinefix")]
    pub mod onlinefix;

    pub mod calc;
    pub mod ping;
    pub mod weather;

    pub mod user;

    pub const DEPARTMENT_NAME: &str = "© SophieUtilities";
}

mod economy {
    pub mod coins;

    pub const DEPARTMENT_NAME: &str = "© SophieEconomy";
}

mod gamble {
    pub mod blackjack;
    pub mod casino;
    pub mod info;

    pub const DEPARTMENT_NAME: &str = "© BetSophie";
}

mod administration {
    pub mod commands;
}

type Command = poise::Command<crate::types::Data, anyhow::Error>;

pub fn commands() -> Vec<Command> {
    let commands = vec![
        economy::coins::coins(),
        fun::jankenpon::jankenpon(),
        fun::message::message(),
        fun::salve::salve(),
        gamble::casino::casino(),
        gamble::blackjack::blackjack(),
        gamble::info::info(),
        utils::ping::ping(),
        utils::calc::calc(),
        utils::weather::weather(),
        utils::user::user(),
        administration::commands::commands(),
    ];

    #[cfg(feature = "onlinefix")]
    let commands = commands
        .into_iter()
        .chain(std::iter::once(utils::onlinefix::onlinefix()))
        .collect();

    commands
}

pub fn context_menu_commands() -> Vec<Command> {
    vec![
        utils::user::avatar_context()
    ]
}

pub fn all_commands() -> Vec<Command> {
    let mut commands = commands();
    commands.extend(context_menu_commands());
    commands
}