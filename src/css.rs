use maud::{html, Markup, Render};

/// Links to a CSS stylesheet at the given path.
pub struct Css(&'static str);

impl Css {
    pub const STYLESHEET: Css = Css("/stylesheet.css");
}

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}
