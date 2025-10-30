use crate::models::{Blog, Chapter};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct Store {
    pub pool: Pool<Postgres>,
}

impl Store {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Old (kept if you still want non-paginated):
    pub async fn list_blogs(&self) -> Result<Vec<Blog>, sqlx::Error> {
        sqlx::query_as::<_, Blog>(r#"
            SELECT id, name, meta_tag, small_description, meta_description,
                   tile_image, banner_image, youtube_video_url, created_on
            FROM blogs
            ORDER BY created_on DESC
        "#)
            .fetch_all(&self.pool)
            .await
    }

    // New: paginated blogs + total count
    pub async fn list_blogs_paginated(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Blog>, i64), sqlx::Error> {
        let rows = sqlx::query_as::<_, Blog>(r#"
            SELECT id, name, meta_tag, small_description, meta_description,
                   tile_image, banner_image, youtube_video_url, created_on
            FROM blogs
            ORDER BY created_on DESC
            OFFSET $1 LIMIT $2
        "#)
            .bind(offset)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let total: (i64,) = sqlx::query_as(r#"SELECT COUNT(*) FROM blogs"#)
            .fetch_one(&self.pool)
            .await?;

        Ok((rows, total.0))
    }

    // Old (kept):
    pub async fn list_chapters(&self, blog_id: Uuid) -> Result<Vec<Chapter>, sqlx::Error> {
        sqlx::query_as::<_, Chapter>(r#"
            SELECT id, blog_id, name, meta_tag, small_description, meta_description,
                   tile_image, banner_image, separate_video, video_url, created_on, description
            FROM chapters
            WHERE blog_id = $1
            ORDER BY created_on DESC
        "#)
            .bind(blog_id)
            .fetch_all(&self.pool)
            .await
    }

    // New: paginated chapters + total count
    pub async fn list_chapters_paginated(
        &self,
        blog_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Chapter>, i64), sqlx::Error> {
        let rows = sqlx::query_as::<_, Chapter>(r#"
            SELECT id, blog_id, name, meta_tag, small_description, meta_description,
                   tile_image, banner_image, separate_video, video_url, created_on, description
            FROM chapters
            WHERE blog_id = $1
            ORDER BY created_on DESC
            OFFSET $2 LIMIT $3
        "#)
            .bind(blog_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let total: (i64,) = sqlx::query_as(r#"
            SELECT COUNT(*) FROM chapters WHERE blog_id = $1
        "#)
            .bind(blog_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((rows, total.0))
    }

    pub async fn get_chapter(
        &self,
        blog_id: Uuid,
        chapter_id: Uuid
    ) -> Result<Chapter, sqlx::Error> {
        sqlx::query_as::<_, Chapter>(r#"
            SELECT id, blog_id, name, meta_tag, small_description, meta_description,
                   tile_image, banner_image, separate_video, video_url, created_on, description
            FROM chapters
            WHERE blog_id = $1 AND id = $2
            LIMIT 1
        "#)
            .bind(blog_id)
            .bind(chapter_id)
            .fetch_one(&self.pool)
            .await
    }
}
