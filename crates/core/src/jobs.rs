use async_trait::async_trait;
use serenity::all::{ChannelId, UserId};

use crate::AppError;

pub struct NewJob {
    pub action: String,
    pub args: String,
    pub schedule: String,
    pub channel_id: ChannelId,
    pub created_by: UserId,
}

pub struct Job {
    pub id: i64,
    pub action: String,
    pub args: String,
    pub schedule: String,
    pub channel_id: ChannelId,
    pub created_by: UserId,
    pub next_run_at: Option<i64>,
    pub created_at: i64,
    pub enabled: bool,
}

#[async_trait]
pub trait JobRepo: Send + Sync {
    async fn create(&self, nj: NewJob) -> Result<Job, AppError>;
    async fn list(&self, cid: &ChannelId) -> Result<Vec<Job>, AppError>;
    async fn delete(&self, jid: i64, cid: &ChannelId) -> Result<bool, AppError>;
}

// fn _dyn_check(_repo: Arc<dyn JobRepo>) {}
