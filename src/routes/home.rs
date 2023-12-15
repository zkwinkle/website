use axum::http::Uri;
use maud::{html, Markup};

use crate::components::headings;

pub async fn home(uri: Uri) -> Markup {
    html! {
        (headings(uri))
        h1 { "Hello, World!" }
        p { "Roboto Lorem Ipsum!" }
    }
}
