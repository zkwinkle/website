use axum::http::Uri;
use maud::{html, Markup};

use crate::STYLESHEET;

/// Tab / Link in the navbar
struct NavLink {
    name: &'static str,
    link: &'static str,
}

const TABS: [NavLink; 2] = [
    NavLink {
        name: "🏡 Home",
        link: "/home",
    },
    NavLink {
        name: "📝 Blog",
        link: "/blog",
    },
];

fn navbar(uri: Uri) -> Markup {
    html! {
        nav class="navbar" {
            ul {
                @for link in TABS {
                    li { a
                        class=@if uri.path().starts_with(link.link)
                            { "nav-active" }
                        href=(link.link)
                        { (link.name) }}
                }
            }
        }
    }
}

/// Generate the site's headings (<head> + header)
pub fn headings(uri: Uri) -> Markup {
    html! {
        head { ( STYLESHEET ) }
        header { (navbar(uri)) }
    }
}
