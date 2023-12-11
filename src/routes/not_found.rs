use axum::{http::StatusCode, response::IntoResponse};
use maud::html;

pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        html! {
            h1 { "404 Page Not Found" }
        },
    )
}
