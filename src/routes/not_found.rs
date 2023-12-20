use axum::http::StatusCode;
use maud::{html, Markup};

use super::extractors::Layout;

pub async fn not_found(layout: Layout) -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        layout.render(html! {
            h1 { "404 Page Not Found :(" }
        }),
    )
}
