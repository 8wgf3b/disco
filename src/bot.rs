use serenity::{
    async_trait,
    http::Http,
    model::gateway::{GatewayIntents, Ready},
    prelude::*,
};
use std::sync::Arc;
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!(user = %ready.user.name, "Bot connected");
    }
}

// Shared state for HTTP handlers
#[derive(Clone)]
pub struct BotState {
    pub http: Arc<Http>,
}

impl BotState {
    pub fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

pub async fn create_client(
    token: String,
) -> Result<(Client, Arc<Http>), Box<dyn std::error::Error>> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let http = Arc::new(Http::new(&token));

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    info!("Discord client created");

    Ok((client, http))
}
