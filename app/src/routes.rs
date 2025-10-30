use crate::config::AppConfig;
use axum::{
    body::Body,
    extract::{FromRef, State},
    http::{Request, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use blog::{handlers::BlogState, store::Store as BlogStore};
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::time;
use tower::limit::ConcurrencyLimitLayer;
use axum::http::{HeaderValue, Method, header::HeaderName};
use tower_http::cors::{Any, CorsLayer};



#[derive(Clone)]
pub struct AppState {
    pub blog_store: BlogStore,
}

impl FromRef<AppState> for BlogState {
    fn from_ref(app: &AppState) -> Self {
        BlogState {
            store: app.blog_store.clone(),
        }
    }
}

async fn timeout_mw(
    State(timeout): State<Duration>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    match time::timeout(timeout, next.run(req)).await {
        Ok(res) => res,
        Err(_) => (StatusCode::REQUEST_TIMEOUT, "request timed out").into_response(),
    }
}

// NOTE: return `Router` (no `<AppState>`)!
pub fn build_app_router(pool: Pool<Postgres>, cfg: &AppConfig) -> Router {
    let app_state = AppState {
        blog_store: BlogStore::new(pool.clone()),
    };

    // This is a *missing-state* subrouter; parent `.with_state` will satisfy it.
    let blog_subrouter = blog::router::router::<AppState>();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);

    Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/blog", blog_subrouter)
        .layer(ConcurrencyLimitLayer::new(cfg.rate_limit_per_window as usize))
        .layer(from_fn_with_state(cfg.request_timeout, timeout_mw)).layer(cors)
        .with_state(app_state) // after this, the type is `Router` (concrete)
}
