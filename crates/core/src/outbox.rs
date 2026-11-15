use serenity::all::{ChannelId, CreateMessage, EditMessage, MessageId, ReactionType};
use tokio::sync::mpsc;

use crate::AppError;

#[derive(Debug)]
pub enum Outbound {
    Send {
        channel_id: ChannelId,
        message: CreateMessage,
    },
    React {
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionType,
    },
    Edit {
        channel_id: ChannelId,
        message_id: MessageId,
        edit: EditMessage,
    },
    Delete {
        channel_id: ChannelId,
        message_id: MessageId,
    },
}

impl Outbound {
    pub fn text(cid: impl Into<ChannelId>, s: impl Into<String>) -> Self {
        Self::Send {
            channel_id: cid.into(),
            message: CreateMessage::new().content(s),
        }
    }
}

#[derive(Clone)]
pub struct OutboxHandle {
    tx: mpsc::Sender<Outbound>,
}

impl OutboxHandle {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<Outbound>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { tx }, rx)
    }

    pub async fn send(&self, msg: Outbound) -> Result<(), AppError> {
        self.tx
            .send(msg)
            .await
            .map_err(|_| AppError::OutboxClosed)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{assert_eq, matches};

    use super::*;

    #[tokio::test]
    async fn send_delivers_to_receiver() {
        let (o, mut r) = OutboxHandle::new(1);
        let t = Outbound::text(21111, "wowo");
        let _ = o.send(t).await;
        if let Ok(Outbound::Send {
            channel_id: cid, ..
        }) = r.try_recv()
        {
            assert_eq!(cid, ChannelId::new(21111))
        } else {
            panic!("expected send")
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn zero_size_outbox_handle() {
        let _ = OutboxHandle::new(0);
    }

    #[tokio::test]
    async fn receiver_dropped_before_send() {
        let (o, r) = OutboxHandle::new(1);
        let t = Outbound::text(21111, "wowo");
        drop(r);
        let r = o.send(t).await;
        let t = matches!(r, Err(AppError::OutboxClosed));
        assert!(t);
    }
}
