mod utils;
use utils::*;

use crate::{
    types::{Context, InvocationData, OnlineFixGame, OnlineFixGameInfo, UnitResult},
    utils::{discord::reply::Reply, onlinefix},
};
use poise::{
    command,
    serenity_prelude::{ComponentInteractionCollector, ComponentInteractionDataKind},
};

/// Get online-fix games
#[command(slash_command, user_cooldown = 60)]
pub async fn onlinefix(
    ctx: Context<'_>,
    #[description = "Game to search"]
    #[min_length = 3]
    #[max_length = 50]
    search: String,
) -> UnitResult {
    let data = ctx.data();

    let client = &data.onlinefix_client;
    let redis = &data.redis;
    let timeout = data.config.timeout.onlinefix;

    let mut message = Reply::with_content(":mag: Procurando jogos...")
        .send(&ctx)
        .await?
        .into_message()
        .await?;

    InvocationData::edit_message(&ctx, message.clone()).await;

    let search = match onlinefix::search(client, redis, &search).await {
        Ok(search) if !search.games.is_empty() => search,
        Ok(_) => {
            return Reply::with_content("Nenhum jogo encontrado :space_invader:")
                .edit_ok(&ctx, &mut message)
                .await;
        }
        Err(err) => {
            err.log_error();
            return Reply::with_content(err.message)
                .edit_ok(&ctx, &mut message)
                .await;
        }
    };

    create_search_reply(&search)
        .content("")
        .edit(&ctx, &mut message)
        .await?;

    let mut selected_game: Option<(OnlineFixGameInfo, &OnlineFixGame)> = None;

    while let Some(interaction) = ComponentInteractionCollector::new(ctx)
        .message_id(message.id)
        .timeout(timeout)
        .await
    {
        match interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { ref values } => {
                let game_path = values.first().unwrap();

                let game_info = match onlinefix::info(client, redis, game_path).await {
                    Ok(game_info) => game_info,
                    Err(err) => {
                        err.log_error();
                        Reply::ephemeral(err.message)
                            .followup(&ctx, &interaction)
                            .await?;
                        continue;
                    }
                };

                if let Some(game) = search.games.iter().find(|game| game.url == game_info.url) {
                    create_info_reply(&game_info, game)
                        .edit(&ctx, &mut message)
                        .await?;

                    selected_game = Some((game_info, game));
                }
            }

            ComponentInteractionDataKind::Button => {
                let action = &interaction.data.custom_id;

                if action == "goback-1" {
                    create_search_reply(&search)
                        .edit(&ctx, &mut message)
                        .await?;
                    continue;
                }

                let Some(current_game) = &selected_game else {
                    continue;
                };

                if action == "goback-2" {
                    create_info_reply(&current_game.0, current_game.1)
                        .edit(&ctx, &mut message)
                        .await?;
                    continue;
                }

                let torrent = match onlinefix::torrent(client, redis, &action).await {
                    Ok(game_info) => game_info,
                    Err(err) => {
                        err.log_error();
                        Reply::ephemeral(err.message)
                            .followup(&ctx, &interaction)
                            .await?;
                        continue;
                    }
                };

                create_torrent_reply(&current_game.0, current_game.1, &torrent)
                    .edit(&ctx, &mut message)
                    .await?;
            }

            _ => continue,
        };
    }

    Reply::default()
        .empty_action_rows()
        .edit_ok(&ctx, &mut message)
        .await
}
