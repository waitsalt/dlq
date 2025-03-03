use axum::Router;

mod post;

pub async fn init() -> Router {
    let post_router = post::init().await;

    Router::new().nest("/post", post_router)
}
