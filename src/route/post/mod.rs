mod get_posts;

use axum::Router;

pub fn init() -> Router {
    Router::new()
    // .route("/", get(hello))
}
