use axum::routing::Router;
use crate::handler::list_blogs;

pub fn router(state: TopicState) -> Router{
    Router::new()
        .route("/list_topics", get(handlers::list_blogs))
        .route("/topic/{id}", get(handlers::list_blog_chapters))
        .route("/topic/{topic_id}/chapter/{chapter_id}", get(handlers::get_one_chapter))
}