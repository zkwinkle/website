use axum::routing::get;
use axum::Router;

mod home;
mod not_found;

/// Create the main `Router` for this app.
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home::home))
        .fallback(not_found::not_found)
    // .layer(server_common::cors(
    //     [
    //         "https://my-url.com",
    //         #[cfg(not(feature = "production"))]
    //         "http://localhost:3000",
    //     ],
    //     [],
    // ))
}
