use crate::commands::{ALL, AppError, Command};
use serenity::all::{
    self, CommandData, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};
pub struct Help;

impl Command for Help {
    const NAME: &'static str = "help";
    const DESCRIPTION: &'static str = "All the slash commands you need";
    async fn execute(&self) -> Result<all::CreateInteractionResponse, AppError> {
        let mut embed = CreateEmbed::new().title("Commands");
        for s in ALL {
            embed = embed.field(format!("/{}", s.0), s.1, false);
        }
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(true),
        ))
    }
}

impl TryFrom<&CommandData> for Help {
    type Error = AppError;
    fn try_from(_value: &CommandData) -> Result<Self, Self::Error> {
        Ok(Help)
    }
}
