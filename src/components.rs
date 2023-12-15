use crate::css::Css;
use maud::{html, Markup};

struct NavLink {
    name: &'static str,
    link: &'static str,
}

const TABS: [NavLink; 2] = [
    NavLink {
        name: "🏡 Home",
        link: "/",
    },
    NavLink {
        name: "📝 Blog",
        link: "/blog",
    },
];

fn navbar() -> Markup {
    html! {
        nav class="navbar" {
            ul {
                @for link in TABS {
                    li { a  href=(link.link) { (link.name) }}
                }
            }
        }
    }
}

pub fn headings() -> Markup {
    html! {
        head { ( Css::STYLESHEET ) }
        header { (navbar()) }
    }
}
