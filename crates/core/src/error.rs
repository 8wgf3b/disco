use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Discord error: {0}")]
    Discord(#[from] serenity::Error),
    #[error("Outbox Closed")]
    OutboxClosed,
    #[error("Storage error: {0}")]
    Storage(#[source] Box<dyn std::error::Error + Send + Sync>),
}
