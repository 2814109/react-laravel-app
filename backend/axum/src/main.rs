use sea_orm::Database;

use crate::presentation::controller::router;
pub mod entity;
pub mod infrastructure;
pub mod presentation;
// use crate::infrastructure::repository::task;
// mod task;
mod work;
#[tokio::main]
async fn main() {
    // task::create_one("test3").await;
    //  let db: DatabaseConnection =
    Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
        .await
        .expect("Database connection failed");

    work::debug();
    work::multiple_println();
    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router::app().into_make_service())
        .await
        .unwrap();
}
