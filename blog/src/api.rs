use axum::Router;
use sqlx::{Pool, Postgres};
use crate::{router, handlers::BlogState, store::Store};

/// Build the blog subrouter with its own state attached.
/// The app only needs this function; it never sees handlers or `BlogState`.
pub fn subrouter(pool: Pool<Postgres>) -> Router<BlogState> {
    let state = BlogState { store: Store::new(pool) };
    router::router().with_state(state)
}
