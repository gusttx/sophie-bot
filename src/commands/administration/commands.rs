use crate::{
    types::{Context, Data, InvocationData, UnitResult},
    utils::discord::{
        action_row::{Button, ButtonsRow},
        embed::Embed,
        reply::Reply,
    },
};
use poise::{
    self, command,
    samples::{register_globally, register_in_guild},
    serenity_prelude::{Command, ComponentInteractionCollector},
};
use std::{collections::HashMap, time::Instant};

const DEPARTMENT_NAME: &str = "© SophieCommands";

#[command(prefix_command, owners_only)]
pub async fn commands(ctx: Context<'_>) -> UnitResult {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    let guild_commands = guild_id.get_commands(&ctx).await?;
    let global_commands = Command::get_global_commands(&ctx).await?;
    let commands = &ctx.framework().options().commands;

    let mut message = create_commands_reply(&guild_commands, &global_commands, &commands)
        .send(&ctx)
        .await?
        .into_message()
        .await?;

    InvocationData::edit_message(&ctx, message.clone()).await;

    while let Some(interaction) = ComponentInteractionCollector::new(&ctx)
        .author_id(ctx.author().id)
        .message_id(message.id)
        .timeout(ctx.data().config.timeout.owner_response)
        .await
    {
        let msg = Reply::ephemeral("<a:loading:1364061243282952212>")
            .followup(&ctx, &interaction)
            .await?;

        let started = Instant::now();

        let reply_msg = match interaction.data.custom_id.as_str() {
            "all" => {
                register_globally(ctx, commands).await?;
                "Comandos registrados globalmente"
            }
            "del-all" => {
                if !global_commands.is_empty() {
                    register_globally::<Data, anyhow::Error>(ctx, &[]).await?;
                }
                "Comandos removidos globalmente"
            }
            "all-guild" => {
                register_in_guild(ctx, &commands, guild_id).await?;
                "Comandos registrados no servidor"
            }
            "del-all-guild" => {
                if !guild_commands.is_empty() {
                    register_in_guild::<Data, anyhow::Error>(ctx, &[], guild_id).await?;
                }
                "Comandos removidos do servidor"
            }
            _ => continue,
        };

        Reply::ephemeral(format!(
            "{} - {}ms",
            reply_msg,
            started.elapsed().as_millis()
        ))
        .edit_followup(&ctx, &interaction, &msg)
        .await?;
    }

    Reply::default()
        .empty_action_rows()
        .edit_ok(&ctx, &mut message)
        .await
}

fn create_commands_reply<U, E>(
    guild_commands: &Vec<Command>,
    global_commands: &Vec<Command>,
    commands: &Vec<poise::Command<U, E>>,
) -> Reply {
    let commands = commands
        .iter()
        .filter(|c| c.slash_action.is_some())
        .collect::<Vec<_>>();

    let server_buttons = ButtonsRow::new()
        .add_green(Button::new("all-guild", "Registrar todos no servidor"))
        .add_red(Button::new("del-all-guild", "Remover todos do servidor"));
    let global_buttons = ButtonsRow::new()
        .add_green(Button::new("all", "Registrar todos globalmente"))
        .add_red(Button::new("del-all", "Remover todos globalmente"));

    let mut commands_map: HashMap<&String, [bool; 3]> = HashMap::with_capacity(commands.len());

    for command in &commands {
        commands_map.entry(&command.name).or_default()[0] = true;
    }
    for command in guild_commands {
        commands_map.entry(&command.name).or_default()[1] = true;
    }
    for command in global_commands {
        commands_map.entry(&command.name).or_default()[2] = true;
    }

    let desc_vec = commands_map
        .iter()
        .map(|(name, map)| {
            let icons = match map {
                [true, false, false] => "\\💾",
                [_, true, false] => "\\🏡",
                [_, false, true] => "\\🌎",
                [_, true, true] => "\\🏡\\🌎",
                _ => unreachable!("Invalid command map"),
            };

            match map[0] {
                true => format!("- [{icons}] {name}"),
                false => format!("- [{icons}] ~~{name}~~"),
            }
        })
        .collect::<Vec<_>>();

    let embed = Embed::new(0x36FF00, ":wrench: Comandos Slash")
        .desc(desc_vec.join("\n"))
        .inline_field("\\💾 Existentes", format!("{} comandos", commands.len()))
        .inline_field(
            "\\🏡 No servidor",
            format!("{} comandos", guild_commands.len()),
        )
        .inline_field(
            "\\🌎 Globais",
            format!("{} comandos", global_commands.len()),
        )
        .field("\\📋 Totais", format!("{} comandos", commands_map.len()))
        .footer(DEPARTMENT_NAME);

    Reply::with_embed(embed)
        .add_action_row(server_buttons.into())
        .add_action_row(global_buttons.into())
}
