use maud::{html, Markup};

pub async fn home() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}
