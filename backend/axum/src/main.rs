use sea_orm::Database;

use crate::presentation::controller::router;
pub mod entity;
pub mod presentation;
mod task;
mod work;
#[tokio::main]
async fn main() {
    task::create_one().await;
    //  let db: DatabaseConnection =
    Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
        .await
        .expect("Database connection failed");

    work::debug();
    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router::app().into_make_service())
        .await
        .unwrap();
}
