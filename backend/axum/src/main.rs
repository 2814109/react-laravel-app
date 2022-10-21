use crate::presentation::controller::router;
pub mod presentation;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router::app().into_make_service())
        .await
        .unwrap();
}
