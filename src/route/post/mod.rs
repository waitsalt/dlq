use axum::{Router, routing::post};

pub async fn init() -> Router {
    Router::new().route("/", post(hello))
}

pub async fn hello() {
    println!("asda");
}
