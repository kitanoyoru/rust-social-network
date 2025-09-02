use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommentStatus {
    Active,
    Hidden,
    Deleted,
    Flagged,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub media_urls: Vec<String>,
    pub like_count: i32,
    pub reply_count: i32,
    pub status: CommentStatus,
    pub hashtags: Vec<String>,
}
