use axum::http::Uri;
use maud::{html, Markup, Render};

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

/// A navbar with certain navlinks
pub struct Navbar {
    links: &'static [NavLink],
    current_uri: Uri,
}

impl Render for Navbar {
    fn render(&self) -> Markup {
        html! {
            nav class="navbar" {
                ul {
                    @for link in self.links {
                        li { a
                            class=@if self.current_uri.path().starts_with(link.link)
                                { "nav-active" }
                            href=(link.link)
                            { (link.name) }}
                    }
                }
            }
        }
    }
}

impl Navbar {
    /// Generate the site's navbar
    pub fn from_uri(uri: Uri) -> Navbar {
        Navbar {
            links: &TABS,
            current_uri: uri,
        }
    }
}
