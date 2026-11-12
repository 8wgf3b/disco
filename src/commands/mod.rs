use std::vec;

use serenity::all::{CommandData, CreateCommand, CreateInteractionResponse};

use crate::commands::ping::Ping;

pub mod ping;

use tracing::info;

pub async fn dispatch(d: &CommandData) -> Result<CreateInteractionResponse, String> {
    info!("got command: {:?}", d.name);

    match d.name.as_str() {
        Ping::NAME => Ping::try_from(d)?.execute().await,
        _ => Err("Unknown Command".to_string()),
    }
}

pub fn register_all() -> Vec<CreateCommand> {
    vec![Ping::register()]
}

pub trait Command {
    const NAME: &'static str;
    async fn execute(&self) -> Result<CreateInteractionResponse, String>;
    fn register() -> CreateCommand;
}
