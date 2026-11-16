use std::sync::Arc;

use disco_core::{AppError, Outbound};
use serenity::all::Http;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

pub async fn run_delivery(mut rx: mpsc::Receiver<Outbound>, http: Arc<Http>) {
    info!("Initiating discord delivery task");
    while let Some(msg) = rx.recv().await {
        if let Err(e) = deliver(&http, msg).await {
            error!("delivery failed: {e}")
        }
    }
    info!("Outbox closed. Ending delivery task")
}

async fn deliver(http: &Http, msg: Outbound) -> Result<(), AppError> {
    match msg {
        Outbound::Send {
            channel_id,
            message,
        } => {
            channel_id.send_message(http, message).await?;
            debug!(%channel_id, "message sent")
        }
        Outbound::Edit {
            channel_id,
            message_id,
            edit,
        } => {
            channel_id.edit_message(http, message_id, edit).await?;
            debug!(%channel_id, "{message_id} is edited.")
        }
        Outbound::Delete {
            channel_id,
            message_id,
        } => {
            channel_id.delete_message(http, message_id).await?;
            debug!(%channel_id, "{message_id} is deleted")
        }
        Outbound::React {
            channel_id,
            message_id,
            emoji,
        } => {
            channel_id
                .create_reaction(http, message_id, emoji.clone())
                .await?;
            debug!(%channel_id, "Reacted with {emoji} for {message_id}")
        }
    };
    Ok(())
}
