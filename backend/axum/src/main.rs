use sea_orm::{Database, DatabaseConnection};

use crate::presentation::controller::router;
pub mod entity;
pub mod infrastructure;
pub mod presentation;
mod work;
#[tokio::main]
async fn main() {
    Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
        .await
        .expect("Database connection failed");

    work::debug();
    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router::app().into_make_service())
        .await
        .unwrap();
}
