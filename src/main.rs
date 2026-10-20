pub mod config;

use std::fs::create_dir_all;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use tracing::{ error, info};
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};

use crate::config::Config;
use serenity::async_trait;
use serenity::client::Context as Ctx;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::Layer;

#[derive(Debug)]
struct Handler;

fn init_logging(log_dir: &str) -> Result<WorkerGuard> {
    create_dir_all(log_dir)?;

    let (file_writer, guard) = tracing_appender::non_blocking(
        daily(log_dir, "disco.jsonl"),
    );    
    let stdout_filter = EnvFilter::new("info");
    let file_filter = EnvFilter::new("debug");
tracing_subscriber::registry()
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
    #[tracing::instrument(skip_all, fields(id = %msg.id, user = %msg.author.name, channel = %msg.channel_id))]
    async fn message(&self, ctx: Ctx, msg: Message) {
        if msg.content == "!ping"
            && let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
        {
            error!("Error sending message: {why:#}");
        }
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
        .event_handler(Handler)
        .await
        .context("Err creating client")
        .inspect_err(|e| error!("{e:#}"))?;

    // Start listening for events by starting a single shard
    client.start().await.inspect_err(|e| error!("Discord client stopped with an error {e:#}"))?; 

    info!("Closing program.");
    Ok(())
}
