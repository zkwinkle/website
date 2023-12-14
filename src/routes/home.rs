use maud::{html, Markup};

use crate::components::headings;

pub async fn home() -> Markup {
    html! {
        (headings())
        h1 { "Hello, World!" }
        p { "Roboto Lorem Ipsum!" }
    }
}
