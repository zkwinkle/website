use axum::http::{StatusCode, Uri};
use maud::{html, Markup};

use crate::components::headings;

pub async fn not_found(uri: Uri) -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        html! {
            (headings(uri))
            h1 { "404 Page Not Found :(" }
        },
    )
}
