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
pub struct Navbar<'a> {
    links: &'static [NavLink],
    current_uri: &'a Uri,
}

struct ThemeSelector;

impl Render for ThemeSelector {
    fn render(&self) -> Markup {
        const THEMES: [(&'static str, &'static str); 3] = [
            ("blue-evening", "Blue Evening"),
            ("pink-fair", "Pink Fair"),
            ("sunny-picnic", "Sunny Picnic"),
        ];

        html! {
            div class="theme-selector" {
                label {
                    i class="fa fa-paint-brush" {}
                }
                div class="theme-popup" {
                    ul {
                        @for theme in THEMES {
                        li class="theme" {
                            input type="radio" id=(theme.0)
onClick={"
    document.getElementById('theme-container').className = '"(theme.0)"'
    localStorage.setItem('theme', '"(theme.0)"');
    "
}
                                name="themes-group";
                            label for=(theme.0) {
                                (theme.1)
                            }
                        }
                        }
                    }
                script {
                    // NOTE: themeClass variable must be defined in Layout
                    // extractor
                    "
                    document.getElementById(themeClass).click();
                    "
                    }
                }
            }
        }
    }
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
                    ( ThemeSelector )
                }
            }
        }
    }
}

impl Navbar<'_> {
    /// Generate the site's navbar
    pub fn from_uri(uri: &Uri) -> Navbar {
        Navbar {
            links: &TABS,
            current_uri: uri,
        }
    }
}
