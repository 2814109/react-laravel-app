use crate::infrastructure::repository::task;
// use axum::response::Json;
use axum::{routing::get, Router};
pub fn app() -> Router {
    let router: Router = Router::new()
        .route("/", get(|| async { "Hello, World! 1" }))
        .route("/test", get(|| async { "Hello, World! 2" }))
        .route("/response", get(|| async { task::create_one("api").await }))
        .route("/task", get(|| async { task::get_task_by_id().await }))
        .route("/tasks", get(|| async { task::get_all_task().await }));

    return router;
}
