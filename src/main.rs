mod commands;
pub mod config;
use std::fs::create_dir_all;
use std::time::Duration;

use anyhow::{Context, Result};
use disco_core::{AppError, Outbound, OutboxHandle};
use disco_discord::run_delivery;
use dotenvy::dotenv;
use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};

use crate::commands::{dispatch, register_all};
use crate::config::Config;
use serenity::async_trait;
use serenity::client::Context as Ctx;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{Layer, Registry};

#[derive(Debug)]
struct Handler {
    guild_id: GuildId,
}

fn init_logging(log_dir: &str) -> Result<WorkerGuard> {
    create_dir_all(log_dir)?;
    let console_layer = console_subscriber::spawn::<Registry>();
    let (file_writer, guard) = tracing_appender::non_blocking(daily(log_dir, "disco.jsonl"));
    let stdout_filter = EnvFilter::new("info");
    let file_filter = EnvFilter::new("debug");
    tracing_subscriber::registry()
        .with(console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(stdout_filter),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(file_writer)
                .with_filter(file_filter),
        )
        .try_init()
        .context("installing tracing subscriber")?;
    Ok(guard)
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Ctx, msg: Message) {
        debug!("{:?}", msg.content)
    }

    async fn ready(&self, ctx: Ctx, ready: Ready) {
        info!("Connected as {:?}", ready.user.name);
        match self.guild_id.set_commands(&ctx.http, register_all()).await {
            Ok(_) => info!("Commands registered"),
            Err(e) => error!("failed to register commands: {e}"),
        }
    }

    async fn interaction_create(&self, ctx: Ctx, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let response = match dispatch(&cmd.data).await {
                Ok(resp) => resp,
                Err(e) => {
                    warn!("command failed: {e:#}");
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(e.to_string())
                            .ephemeral(true),
                    )
                }
            };
            let _ = cmd
                .create_response(&ctx.http, response)
                .await
                .inspect_err(|e| error!("failed to reply: {e}"));
        }
    }
}

async fn tick_producer(outbox: OutboxHandle, channel: u64) {
    info!("Initiated tick producer task @ {channel}");
    loop {
        let msg = Outbound::text(channel, "tock!");
        if let Err(AppError::OutboxClosed) = outbox.send(msg).await {
            info!("Outbox is closed. Closing the tick producer");
            break;
        }
        sleep(Duration::from_mins(15)).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv().context("No local .env file");
    let config = Config::from_env()?;
    info!("Config loaded");
    let _log_guard = init_logging(&config.log_dir)?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    //println!("{}", config.discord_token);
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(Handler {
            guild_id: GuildId::new(config.guild_id),
        })
        .await
        .context("Err creating client")
        .inspect_err(|e| error!("{e:#}"))?;

    // Start listening for events by starting a single shard
    let (outbox, rx) = OutboxHandle::new(100);
    let mut jset = JoinSet::new();
    jset.spawn(run_delivery(rx, client.http.clone()));
    jset.spawn(tick_producer(outbox, config.tick_cid));
    client
        .start()
        .await
        .inspect_err(|e| error!("Discord client stopped with an error {e:#}"))?;
    info!("Closing program.");
    Ok(())
}
