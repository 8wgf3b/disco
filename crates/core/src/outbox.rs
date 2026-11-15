use serenity::all::{ChannelId, CreateMessage, EditMessage, MessageId, ReactionType};

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
    pub fn text(cid: ChannelId, s: impl Into<String>) -> Self {
        Self::Send {
            channel_id: cid,
            message: CreateMessage::new().content(s),
        }
    }
}

pub struct OutboxHandle {
    capacity: usize,
}
