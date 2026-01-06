use tracing::{debug, error, info};

mod api;
mod bot;
mod models;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Discord bot with HTTP API...");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");
    let http_port = std::env::var("HTTP_PORT").unwrap_or_else(|_| "3000".to_string());
    let http_port = http_port
        .parse::<u16>()
        .expect("HTTP_PORT must be a valid port number");

    debug!("Discord token loaded from environment");
    debug!("HTTP port: {}", http_port);

    // Create Discord client and get HTTP handle
    let (mut client, http) = bot::create_client(token)
        .await
        .expect("Failed to create client");

    // Create shared state for HTTP handlers
    let bot_state = bot::BotState::new(http);

    // Run both bot and HTTP server concurrently
    let bot_handle = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            error!(?why, "Discord bot error");
        }
    });

    let api_handle = tokio::spawn(async move {
        api::run_server(bot_state, http_port).await;
    });

    // Wait for both to complete (or one to fail)
    tokio::select! {
        _ = bot_handle => {
            info!("Bot task completed");
        },
        _ = api_handle => {
            info!("API task completed");
        }
    }

    info!("Shutdown complete");
}
