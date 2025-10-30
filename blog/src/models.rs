use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct ListBlogs {
    pub data: Vec<Blog>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: Uuid,
    pub name: String,
    pub meta_tag: String,
    pub small_description: String,
    pub meta_description: String,
    pub tile_image: String,
    pub banner_image: String,
    pub youtube_video_url: Option<String>,
    pub created_on: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChapters {
    pub data: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Chapter {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub name: String,
    pub meta_tag: String,
    pub small_description: String,
    pub meta_description: String,
    pub tile_image: String,
    pub banner_image: String,
    pub separate_video: bool,
    pub video_url: Option<String>,     // <-- changed from bool to Option<String>
    pub created_on: DateTime<Utc>,
    pub description: Option<String>,
}