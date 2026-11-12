use anyhow::{Context, Result};
use std::env::var;

pub struct Config {
    pub discord_token: String,
    pub log_dir: String,
    pub guild_id: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let discord_token = var("DISCORD_TOKEN").context(
            "DISCORD_TOKEN is not set; set it in the environment or in a local .env file",
        )?;
        let log_dir = var("LOG_DIR")
            .context("LOG_DIR envvar is not set. Defaulting to ./logs/")
            .unwrap_or("logs/".to_owned());
        let guild_id = var("GUILD_ID")
            .context("Main Server GUILD_ID env var not set.")?
            .parse()?;
        Ok(Self {
            discord_token,
            log_dir,
            guild_id,
        })
    }
}
