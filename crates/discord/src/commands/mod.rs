pub mod help;
pub mod ping;

use std::vec;

use serenity::all::{CommandData, CreateCommand, CreateInteractionResponse};

use crate::commands::{help::Help, ping::Ping};
use disco_core::AppError;

use tracing::info;

pub const ALL: &[(&str, &str)] = &[
    (Help::NAME, Help::DESCRIPTION),
    (Ping::NAME, Ping::DESCRIPTION),
];

pub async fn dispatch(d: &CommandData) -> Result<CreateInteractionResponse, AppError> {
    info!("got command: {:?}", d.name);

    match d.name.as_str() {
        Ping::NAME => Ping::try_from(d)?.execute().await,
        Help::NAME => Help::try_from(d)?.execute().await,
        x => Err(AppError::UnknownCommand(x.to_string())),
    }
}

pub fn register_all() -> Vec<CreateCommand> {
    vec![Ping::register(), Help::register()]
}

pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    fn execute(
        &self,
    ) -> impl std::future::Future<Output = Result<CreateInteractionResponse, AppError>> + Send;
    fn register() -> CreateCommand {
        CreateCommand::new(Self::NAME).description(Self::DESCRIPTION)
    }
}
