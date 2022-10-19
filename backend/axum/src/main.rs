use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" })).route("/test", get(|| async { "Hello, World! 2" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}