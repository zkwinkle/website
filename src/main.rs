use website::{create_app_config_from_env, create_router};

#[tokio::main]
async fn main() {
    let config = create_app_config_from_env();
    let router = create_router(config);

    axum::Server::bind(&"0.0.0.0:31415".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
