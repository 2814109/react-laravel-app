use axum::{
    routing::get,
    Router,
    // routing::BoxRoute
};

pub fn app() -> Router {
    let router: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test", get(|| async { "Hello, World! 2" }));
    return router;
}
