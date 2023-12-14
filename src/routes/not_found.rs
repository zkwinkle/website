use axum::http::StatusCode;
use maud::{html, Markup};

pub async fn not_found() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        html! {
            h1 { "404 Page Not Found :(" }
        },
    )
}
