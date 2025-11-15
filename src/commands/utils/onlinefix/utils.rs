use crate::{
    types::{OnlineFixGame, OnlineFixGameInfo, OnlineFixSearch, OnlineFixTorrent},
    utils::{
        discord::{
            action_row::{Button, ButtonsRow, StringSelectMenu},
            embed::Embed,
            reply::Reply,
        },
        format_bytes,
        onlinefix::ONLINEFIX_LOGO_URL,
    },
};

const DEPARTMENT_NAME: &str = "© Sophie Fix";
const EMBED_COLOR: u32 = 0x0474BC;

pub fn create_search_reply(search: &OnlineFixSearch) -> Reply {
    let mut select_menu = StringSelectMenu::new("game-select", search.games.len());
    let mut fields = Vec::with_capacity(search.games.len());

    for (i, game) in search.games.iter().enumerate() {
        let url = game.url.split("games/").nth(1).unwrap();
        let title = format!("{}. {}", i + 1, game.title);
        let field_value = format!(
            ":calendar: {} | :eye: {} \n :link: [Visitar]({})",
            game.release_date, game.views, game.url
        );

        select_menu.add_option(&title, url);
        fields.push((title, field_value));
    }

    let embed = Embed::new(EMBED_COLOR, ":globe_with_meridians: Online Fix")
        .url(&search.search_url)
        .desc(format!("Jogos encontrados com a busca `{}`", search.input))
        .small_image(ONLINEFIX_LOGO_URL)
        .fields(fields)
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed)
        .content("")
        .add_action_row(select_menu.into())
}

pub fn create_info_reply(game_info: &OnlineFixGameInfo, game: &OnlineFixGame) -> Reply {
    let buttons = ButtonsRow::new()
        .add_grey(Button::new("goback-1", "Voltar"))
        .add_green(Button::new(&game_info.download, "Torrent"));

    let embed = Embed::new(EMBED_COLOR, format!(":video_game: {}", game.title))
        .url(&game_info.url)
        .small_image(ONLINEFIX_LOGO_URL)
        .field(
            "Data de lançamento",
            format!(":calendar: {}", game.release_date),
        )
        .field("Visualizações", format!(":eye: {}", game.views))
        .field("Versão", format!(":package: {}", game_info.build))
        .large_image(&game.image)
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed).add_action_row(buttons.into())
}

pub fn create_torrent_reply(
    info: &OnlineFixGameInfo,
    game: &OnlineFixGame,
    torrent: &OnlineFixTorrent,
) -> Reply {
    let buttons = ButtonsRow::new().add_grey(Button::new("goback-2", "Voltar"))
        .add_link(format!("https://r.gustta.dev/{}", torrent.magnet), "Magnet");

    let size = torrent.files.iter().map(|file| file.length).sum();

    let embed = Embed::new(EMBED_COLOR, format!(":pirate_flag: {}", torrent.name))
        .url(&info.url)
        .small_image(ONLINEFIX_LOGO_URL)
        .field("Versão", format!(":package: {}", &info.build))
        .field("Tamanho", format!(":file_folder: {}", format_bytes(size)))
        .field("Senha", ":key: online-fix.me")
        .field("Link magnético", &torrent.magnet)
        .large_image(&game.image)
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed).add_action_row(buttons.into())
}
