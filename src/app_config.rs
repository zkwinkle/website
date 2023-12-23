use std::path::Path;

/// Configuration for the server.
pub struct AppConfig {
    /// Directory that holds static files like css and icons
    pub public_dir: &'static Path,
}

/// Create AppConfig to use when running the server binary.
pub fn create_app_config_from_env() -> AppConfig {
    let public_dir_str: &'static str = Box::new(read_env_public_dir()).leak();
    let public_dir = Path::new(public_dir_str);

    AppConfig { public_dir }
}

/// Read an environment variable, falling back on the default value only if the `production`
/// feature flag is unset, otherwise panicking.
fn read_env(var_name: &str, default_value_dev: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| {
        if cfg!(feature = "production") {
            panic!("Missing environment variable: {var_name}");
        }
        default_value_dev.to_owned()
    })
}

/// # Panics
///
/// Panics when the "production" feature is enabled and the environment variable is not found.
fn read_env_public_dir() -> String { read_env("PUBLIC_DIR", "public") }
