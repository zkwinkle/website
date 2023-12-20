use maud::{html, Markup};

use super::extractors::Layout;

pub async fn blog(layout: Layout) -> Markup {
    layout.render(html! { h1{ "WIP" } })
}
