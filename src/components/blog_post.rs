use maud::{html, Markup, Render};
use time::{macros::date, Date};

use crate::components::markdown::Markdown;

/// Tab / Link in the navbar
pub struct BlogPost {
    title: &'static str,
    date: Date,
    contents: &'static str,
}

pub const BLOG_POSTS: [BlogPost; 0] = [];
// [BlogPost {
//     contents: include_str!("blog_posts/First post.md"),
//     date: date!(2023 - 12 - 20),
//     title: "First post!",
// }];

impl Render for BlogPost {
    fn render(&self) -> Markup {
        html! {
            h1 { (self.title) }
            ( Markdown(self.contents) )
        }
    }
}
