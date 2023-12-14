use maud::{html, Markup};

use crate::css::Css;

pub async fn home() -> Markup {
    html! {
        head { ( Css::STYLESHEET ) }
        h1 { "Hello, World!" }
        p { "Blue text!" }
    }
}
