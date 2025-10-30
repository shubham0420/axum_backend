use axum::{routing::get, Router};
use crate::handlers::{self, BlogState};

/// Generic router that works with any app state `S`
/// as long as `BlogState` can be derived from `S` via `FromRef`.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    BlogState: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/list_blogs", get(handlers::list_blogs))
        .route("/blog/{id}", get(handlers::list_blog_chapters))
        .route("/blog/{blog_id}/chapter/{chapter_id}", get(handlers::get_one_chapter))
}
