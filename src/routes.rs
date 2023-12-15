use axum::handler::HandlerWithoutStateExt;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

mod home;
mod not_found;

/// Create the main `Router` for this app.
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::permanent("/home") }))
        .route("/home", get(home::home))
        .fallback_service(ServeDir::new("public").fallback(not_found::not_found.into_service()))
}
