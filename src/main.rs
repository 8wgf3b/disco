pub mod config;
use std::fs::create_dir_all;
use std::time::Duration;

use anyhow::{Context, Result};
use disco_core::{AppError, Outbound, OutboxHandle};
use disco_discord::run_delivery;
use dotenvy::dotenv;

use serenity::Client;
use serenity::all::GatewayIntents;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::{error, info};
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};

use crate::config::Config;
use disco_discord::Handler;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{Layer, Registry};

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
        .event_handler(Handler::new(config.guild_id))
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
