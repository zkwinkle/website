#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod app_config;
mod components;
mod css;
mod routes;

pub use app_config::create_app_config_from_env;
pub use css::STYLESHEET;
pub use routes::create_router;
