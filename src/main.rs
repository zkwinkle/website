mod components;
mod css;
mod routes;

#[tokio::main]
async fn main() {
    let router = routes::create_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
