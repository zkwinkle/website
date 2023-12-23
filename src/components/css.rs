use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use maud::{html, Markup, Render};

/// Main stylesheet used throughout the website
pub const STYLESHEET: Css = Css("/stylesheet.css");

/// Font Awesome stylesheet
pub const FONT_AWESOME: Css = Css("/font-awesome-4.7.0/css/font-awesome.min.css");

/// Links to a CSS stylesheet at the given path.
pub struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        let mut link = String::from(self.0);

        if !cfg!(feature = "production") {
            if let Ok(n) = SystemTime::now().duration_since(UNIX_EPOCH) {
                link.push('?');
                link.push_str(&n.as_secs().to_string());
            }
        }

        html! {
            link rel="stylesheet" type="text/css" href=(link);
        }
    }
}
