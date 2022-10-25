use crate::infrastructure::repository::task;
use hyper::Method;
use tower_http::cors::{CorsLayer, AllowOrigin};

// use axum::response::Json;
use axum::{routing::get, Router};
pub fn app() -> Router {
    let router: Router = Router::new()
        .route("/", get(|| async { "Hello, World! 1" }))
        .route("/test", get(|| async { "Hello, World! 2" }))
        .route("/new_task", get(|| async { task::create_one("api").await }))
        .route("/task", get(|| async { task::get_task_by_id().await }))
        .route("/tasks", get(|| async { task::get_all_task().await }))
      .layer(
     CorsLayer::new()
         .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
         .allow_methods(vec![Method::GET]),);

    return router;
}
