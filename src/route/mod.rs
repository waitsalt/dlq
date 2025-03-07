use axum::Router;
use tower_http::trace;

mod post;
mod user;

pub fn init() -> Router {
    let post_router = post::init();
    let user_router = user::init();

    Router::new()
        .nest("/post", post_router)
        .nest("/user", user_router)
        .layer(trace::TraceLayer::new_for_http())
}
