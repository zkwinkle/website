use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

mod home;
mod not_found;

/// Create the main `Router` for this app.
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home::home))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found::not_found)
}
