use crate::commands::{AppError, Command};
use serenity::all::{
    self, CommandData, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub struct Ping;

impl Command for Ping {
    const NAME: &'static str = "ping";
    const DESCRIPTION: &'static str = "replies 'pong!'";
    async fn execute(&self) -> Result<all::CreateInteractionResponse, AppError> {
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("pong!"),
        ))
    }
}

impl TryFrom<&CommandData> for Ping {
    type Error = AppError;
    fn try_from(_value: &CommandData) -> Result<Self, Self::Error> {
        Ok(Ping)
    }
}
