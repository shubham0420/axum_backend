use axum::{extract::{Path, State, Query}, Json};
use uuid::Uuid;
use serde::Serialize;
use crate::{
    models::{Chapter, Blog},
    store::Store,
};
use utils::http_error::ApiError;
use utils::pagination::{PageParams, Pagination};

#[derive(Clone)]
pub struct BlogState { pub store: Store }


pub async fn list_blogs(
    State(s): State<BlogState>,
    Query(params): Query<PageParams>,
) -> Result<Json<Paginated<Blog>>, ApiError> {
    let p: Pagination = params.try_into()?;

    let (rows, total) = s.store
        .list_blogs_paginated(p.offset, p.per_page as i64)
        .await
        .map_err(ApiError::from)?;

    let has_more = (p.offset + rows.len() as i64) < total;
    Ok(Json(Paginated { data: rows, page: p.page, per_page: p.per_page, total, has_more }))
}


pub async fn list_blog_chapters(
    State(s): State<BlogState>,
    Path(blog_id): Path<Uuid>,
    Query(params): Query<PageParams>,
) -> Result<Json<Paginated<Chapter>>, ApiError> {
    let p: Pagination = params.try_into()?;

    let (rows, total) = s.store
        .list_chapters_paginated(blog_id, p.offset, p.per_page as i64)
        .await
        .map_err(ApiError::from)?;

    let has_more = (p.offset + rows.len() as i64) < total;
    Ok(Json(Paginated { data: rows, page: p.page, per_page: p.per_page, total, has_more }))
}


pub async fn get_one_chapter(
    State(s): State<BlogState>,
    Path((blog_id, chapter_id)): Path<(Uuid, Uuid)>
) -> Result<Json<Chapter>, ApiError> {
    let chapter = s.store.get_chapter(blog_id, chapter_id).await.map_err(|e| {
        match e {
            sqlx::Error::RowNotFound => ApiError::NotFound { resource: "chapter", id: chapter_id.to_string() },
            _ => ApiError::from(e),
        }
    })?;
    Ok(Json(chapter))
}


#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub has_more: bool,
}