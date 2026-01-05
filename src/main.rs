use serenity::{
    async_trait,
    model::gateway::{GatewayIntents, Ready},
    prelude::*,
};
use tracing::{debug, error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!(user = %ready.user.name, "Bot connected");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Discord bot...");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");
    debug!("Discord token loaded from environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    debug!(?intents, "Gateway intents configured");

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    info!("Client created successfully, starting...");

    if let Err(why) = client.start().await {
        error!(?why, "Client error");
    }
}
