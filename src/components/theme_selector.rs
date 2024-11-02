use maud::{html, Markup, Render};

pub struct ThemeSelector;

impl Render for ThemeSelector {
    fn render(&self) -> Markup {
        const THEMES: [(&'static str, &'static str); 3] = [
            ("blue-evening", "Blue Evening"),
            ("pink-fair", "Pink Fair"),
            ("sunny-picnic", "Sunny Picnic"),
        ];

        #[rustfmt::skip]
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
