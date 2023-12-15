use axum::http::StatusCode;
use maud::{html, Markup};

use crate::components::headings;

pub async fn not_found() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        html! {
            (headings())
            h1 { "404 Page Not Found :(" }
        },
    )
}
