use crate::components::blog_post::BLOG_POSTS;
use axum::{routing::get, Router};
use maud::{html, Markup};

use super::extractors::Layout;

pub fn create_blog_router() -> Router {
    let mut router = Router::new().route("/", get(blog));

    for post in BLOG_POSTS {
        router = router.route(
            &format!("/{}", linkify_title(post.title)),
            get(|layout: Layout| async move { layout.render(html! { ( post ) }) }),
        )
    }

    router
}

async fn blog(layout: Layout) -> Markup {
    layout.render(html! {
        h1 class="blog-title" { "zkwinkle's blog🗒️" }
        @for blog in &BLOG_POSTS {
            div class="post-preview" {
                h2  { a href=(format!("/blog/{}",linkify_title(blog.title))) { (blog.title) } }
                time datetime=(blog.date) { (blog.date) }
            }
        }
    })
}

fn linkify_title(title: &str) -> String {
    let mut url_title = title.split_ascii_whitespace().collect::<Vec<_>>().join("_");

    url_title.make_ascii_lowercase();

    url_title
}
