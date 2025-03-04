use axum::Router;
use tower_http::trace;

mod post;

pub fn init() -> Router {
    let post_router = post::init();

    Router::new()
        .nest("/post", post_router)
        .layer(trace::TraceLayer::new_for_http())
}
