use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, Uri},
};
use maud::{html, Markup};

use crate::{components::navbar, STYLESHEET};

/// Defines the base layout of a page that will wrap its contents with container
/// divs, headers, footers.
///
/// Usage:
/// ```ignore
/// async fn endpoint(layout: Layout) -> Markup {
///    layout.render(html! { "Hello, World!" })
/// }
/// ```
pub struct Layout {
    uri: Uri,
}

#[async_trait]
impl<S> FromRequestParts<S> for Layout
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            uri: parts.uri.clone(),
        })
    }
}

impl Layout {
    pub fn render(self, content: Markup) -> Markup {
        html! {
            head { ( STYLESHEET ) }
            div class="container" {
                header { (navbar(self.uri)) }
                (content)
            }
        }
    }
}
