use crate::commands::Command;
use serenity::all::{
    self, CommandData, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub struct Ping;

impl Command for Ping {
    const NAME: &'static str = "ping";
    async fn execute(&self) -> Result<all::CreateInteractionResponse, String> {
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("pong!"),
        ))
    }

    fn register() -> CreateCommand {
        CreateCommand::new("ping").description("Replies pong")
    }
}

impl TryFrom<&CommandData> for Ping {
    type Error = String;
    fn try_from(_value: &CommandData) -> Result<Self, Self::Error> {
        Ok(Ping)
    }
}
