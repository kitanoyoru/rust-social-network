use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PostType {
    Text,
    Image,
    Video,
    Link,
    Poll,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PostVisibility {
    Public,
    Followers,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub post_type: PostType,
    pub media_urls: Vec<String>,
    pub visibility: PostVisibility,
    pub like_count: u64,
    pub comment_count: u64,
    pub share_count: u64,
    pub view_count: u64,
    pub tags: Vec<String>,
    pub loc: Option<String>,
}

