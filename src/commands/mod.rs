mod fun {
    pub mod jankenpon;
    pub mod message;
    pub mod salve;

    pub const DEPARTMENT_NAME: &str = "© SophieParty";
}

mod utils {
    #[cfg(feature = "onlinefix")]
    pub mod onlinefix;

    pub mod ping;
    pub mod calc;
    pub mod weather;

    pub const DEPARTMENT_NAME: &str = "© SophieUtilities";
}

mod economy {
    pub mod coins;

    pub const DEPARTMENT_NAME: &str = "© SophieEconomy";
}

mod gamble {
    pub mod casino;
    pub mod blackjack;
    pub mod info;

    pub const DEPARTMENT_NAME: &str = "© BetSophie";
}


pub fn all_commands() -> Vec<poise::Command<crate::types::Data, anyhow::Error>> {
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
    ];

    #[cfg(feature = "onlinefix")]
    let commands = commands.into_iter()
        .chain(std::iter::once(utils::onlinefix::onlinefix()))
        .collect();

    commands
}