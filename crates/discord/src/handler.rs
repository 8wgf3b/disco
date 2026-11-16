use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::client::Context as Ctx;
use serenity::model::channel::Message;
use serenity::prelude::*;

use tracing::{debug, error, info, warn};

use crate::commands::{dispatch, register_all};

#[derive(Debug)]
pub struct Handler {
    guild_id: GuildId,
}

impl Handler {
    pub fn new(gid: u64) -> Self {
        Self {
            guild_id: gid.into(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Ctx, msg: Message) {
        debug!("{:?}", msg.content)
    }

    async fn ready(&self, ctx: Ctx, ready: Ready) {
        info!("Connected as {:?}", ready.user.name);
        match self.guild_id.set_commands(&ctx.http, register_all()).await {
            Ok(_) => info!("Commands registered"),
            Err(e) => error!("failed to register commands: {e}"),
        }
    }

    async fn interaction_create(&self, ctx: Ctx, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let response = match dispatch(&cmd.data).await {
                Ok(resp) => resp,
                Err(e) => {
                    warn!("command failed: {e:#}");
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(e.to_string())
                            .ephemeral(true),
                    )
                }
            };
            let _ = cmd
                .create_response(&ctx.http, response)
                .await
                .inspect_err(|e| error!("failed to reply: {e}"));
        }
    }
}
