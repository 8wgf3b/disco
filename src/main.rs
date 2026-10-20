pub mod config;

use anyhow::Context;
use dotenvy::dotenv;

use crate::config::Config;
use serenity::async_trait;
use serenity::client::Context as Ctx;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Ctx, msg: Message) {
        if msg.content == "!ping"
            && let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
        {
            println!("Error sending message: {why:?}");
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv().context("No local .env file");
    let config = Config::from_env()?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    //println!("{}", config.discord_token);
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
