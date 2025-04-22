use crate::{
    commands,
    config::get_config,
    database::User,
    types::{Context, Data, InvocationData, UnitResult},
    utils::discord::reply::Reply,
};
use anyhow::Error;
use log::{error, info, warn};
use poise::{
    serenity_prelude::{self, ActivityData, FullEvent, Interaction},
    Context as PoiseContext, EditTracker, Framework, FrameworkError, FrameworkOptions,
    PrefixFrameworkOptions,
};
use std::{ops::DerefMut, sync::Arc};

pub fn create_framework(data: Data) -> Framework<Data, Error> {
    let config = &get_config().bot;

    let prefix = config.prefix.clone();
    let edit_tracker = EditTracker::for_timespan(config.edit_tracker_duration);

    Framework::builder()
        .options(FrameworkOptions {
            owners: get_config().owner_ids.clone(),
            commands: commands::all_commands(),
            on_error: |err| Box::pin(on_error(err)),
            pre_command: |ctx| Box::pin(pre_command(ctx)),
            skip_checks_for_owners: true,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(prefix),
                edit_tracker: Some(Arc::new(edit_tracker)),
                ..Default::default()
            },
            event_handler: |ctx, event, _, _| Box::pin(event_handler(ctx, event)),
            ..Default::default()
        })
        .setup(|ctx, ready, _framework| {
            Box::pin(async move {
                let activity = ActivityData::competing("main.rs");
                ctx.set_activity(Some(activity));

                info!("Bot logged in as {}", ready.user.name);

                Ok(data)
            })
        })
        .build()
}

async fn pre_command(ctx: PoiseContext<'_, Data, Error>) -> () {
    let author = ctx.author();

    info!(
        "{}/{} used command '{}'",
        author.name,
        author.id,
        ctx.command().qualified_name
    );
}

async fn event_handler(ctx: &serenity_prelude::Context, event: &FullEvent) -> UnitResult {
    match event {
        FullEvent::InteractionCreate { interaction } => {
            if let Interaction::Component(interaction) = interaction {
                interaction.defer(ctx).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn on_error(error: FrameworkError<'_, Data, Error>) -> () {
    match error {
        FrameworkError::CooldownHit {
            ctx,
            remaining_cooldown,
            ..
        } => {
            let message = format!(
                "Espere **{}.{} segundo(s)** antes de usar esse comando novamente",
                remaining_cooldown.as_secs(),
                remaining_cooldown.as_millis() % 1000,
            );

            if let Err(err) = Reply::ephemeral(message).send(&ctx).await {
                error!("Error sending cooldown message: {}", err);
            }
        }

        FrameworkError::Command { error, ctx, .. } => {
            if error.to_string().contains("Unknown Message") {
                return;
            }

            error!("Command error: {}", error);

            if let Some(mut data) = ctx.invocation_data::<InvocationData>().await {
                if handle_with_invocation_data(&ctx, data.deref_mut()).await {
                    return;
                }
            }

            let message = ":koala: Ops, algo deu errado!";
            _ = Reply::ephemeral(message).send(&ctx).await;
        }

        FrameworkError::ArgumentParse { ctx, .. } => {
            let cmd = ctx.command();

            let params = cmd
                .parameters
                .iter()
                .map(|p| format!("<{}>", p.name))
                .collect::<Vec<_>>()
                .join(" ");

            let phrase = format!(
                "**USE:** ``{}{} {}``",
                ctx.prefix(),
                cmd.qualified_name,
                params
            );

            if let Err(err) = Reply::with_content(phrase).send(&ctx).await {
                error!("Error sending argument parse error message: {}", err);
            }
        }

        FrameworkError::CommandPanic { ctx, .. } => {
            error!("Panic: {}", error);

            if let Some(mut data) = ctx.invocation_data::<InvocationData>().await {
                handle_with_invocation_data(&ctx, data.deref_mut()).await;
                return;
            }

            let message = ":koala: Ops, algo deu errado!";
            if let Err(err) = Reply::ephemeral(message).send(&ctx).await {
                error!("Error sending panic message: {}", err);
            }
        }

        FrameworkError::NotAnOwner { .. }
        | FrameworkError::UnknownCommand { .. }
        | FrameworkError::SubcommandRequired { .. } => {}

        _ => error!("Error: {}", error),
    }
}

async fn handle_with_invocation_data<'a>(ctx: &Context<'_>, data: &'a mut InvocationData) -> bool {
    if !data.coins_to_refound.is_empty() {
        let db = &ctx.data().database;
        warn!("Trying to restore coins...");

        for &(user_id, coins) in &data.coins_to_refound {
            _ = User::add_coins(db, user_id, coins).await;
        }
    }

    if let Some(msg) = data.message_to_edit.as_mut() {
        _ = Reply::empty()
            .content(":koala: Ops, algo deu errado!")
            .edit(&ctx, msg)
            .await;

        return true;
    }

    false
}
