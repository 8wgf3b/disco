use poise::serenity_prelude as serenity;
use serenity::model::gateway::GatewayIntents;
use std::sync::Arc;
use tracing::info;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

// Shared state for HTTP handlers (still needed for API)
#[derive(Clone)]
pub struct BotState {
    pub http: Arc<serenity::http::Http>,
}

impl BotState {
    pub fn new(http: Arc<serenity::http::Http>) -> Self {
        Self { http }
    }
}

/// Get the current channel ID
#[poise::command(slash_command, prefix_command)]
async fn cid(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Channel ID: {}", ctx.channel_id())).await?;
    Ok(())
}

pub async fn create_client(token: String) -> Result<(serenity::Client, Arc<serenity::http::Http>), Box<dyn std::error::Error>> {
    let http = Arc::new(serenity::http::Http::new(&token));

    // Get test guild ID from env if available, otherwise use global registration
    let dev_guild_id = std::env::var("DEV_GUILD_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok())
        .map(serenity::model::id::GuildId::new);

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                if let Some(guild_id) = dev_guild_id {
                    // Fast registration for testing
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                    info!("Slash commands registered in guild {}", guild_id);
                } else {
                    // Global registration (slower, up to 1 hour to propagate)
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    info!("Slash commands registered globally");
                }
                Ok(())
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![cid()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await?;

    info!("Discord client created with Poise framework");

    Ok((client, http))
}
