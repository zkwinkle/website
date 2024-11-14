use axum::http::Uri;
use maud::{html, Markup, Render};

/// Tab / Link in the navbar
struct NavLink {
    name: &'static str,
    link: &'static str,
}

const TABS: &'static [NavLink] = &[
    NavLink {
        name: "🏡 Home",
        link: "/home",
    },
    NavLink {
        name: "📝 Blog",
        link: "/blog",
    },
    NavLink {
        name: "📊 µwgpu",
        link: "/uwgpu",
    },
];

/// A navbar with certain navlinks
pub struct Navbar<'a> {
    links: &'static [NavLink],
    current_uri: &'a Uri,
}

impl Render for Navbar<'_> {
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

impl Navbar<'_> {
    /// Generate the site's navbar
    pub fn from_uri(uri: &Uri) -> Navbar {
        Navbar {
            links: TABS,
            current_uri: uri,
        }
    }
}
