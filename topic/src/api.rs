use axum::Router;
use sqlx::{Pool, Postgres};
use crate::{router, handlers::TopicState, store::Store}; 

pub fn subrouter(pool: Pool<Postgres>) -> Router<BlogState> {
    let state = BlogState { store: Store::new(pool)};
    router::router().with_state(state)
}