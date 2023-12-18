use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use maud::{html, Markup, Render};

pub const STYLESHEET: Css = Css("/stylesheet.css");

/// Links to a CSS stylesheet at the given path.
pub struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        let link_with_timestamp: String = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                // Add timestamp in development to reload stylesheet
                if !cfg!(feature = "production") {
                    format!("{}?{}", self.0, n.as_secs().to_string())
                } else {
                    self.0.to_owned()
                }
            }
            Err(_) => String::from(self.0),
        };

        html! {
            link rel="stylesheet" type="text/css" href=(link_with_timestamp);
        }
    }
}
