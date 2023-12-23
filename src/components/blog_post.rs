use maud::{html, Markup, Render};
use time::{macros::date, Date};

use crate::components::markdown::Markdown;

/// Tab / Link in the navbar
#[derive(Clone)]
pub struct BlogPost {
    pub title: &'static str,
    pub date: Date,
    pub contents: &'static str,
}

pub const BLOG_POSTS: [BlogPost; 1] = [BlogPost {
    contents: include_str!("blog_posts/configuring nixos server.md"),
    date: date!(2023 - 12 - 23),
    title: "Setting up my NixOS server",
}];

impl Render for BlogPost {
    fn render(&self) -> Markup {
        html! {
            div class="blog-post" {
                div class="blog-post-header" {
                    h1 { (self.title) }
                }
                ( Markdown(self.contents) )
            }
        }
    }
}
