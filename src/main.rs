pub mod commands;
pub mod config;
pub mod errors;
pub mod events;
pub mod utils;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

use crate::config::{load_config, Config};
use crate::errors::on_error;
use crate::events::on_event;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

pub struct Data {
    config: Config,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Load the config file from config.toml
    let config = load_config();
    // The Discord token is stored in Secrets.toml
    // Shuttle can also read a dev token from Secrets.dev.toml
    let discord_token = secret_store.get("TOKEN").context("'TOKEN' was not found")?;

    let commands = vec![
        commands::registrations::register(),
        commands::registrations::approve(),
        commands::jobs::job(),
    ];

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.general.prefix.clone()),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { config })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
