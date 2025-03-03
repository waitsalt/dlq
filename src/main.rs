mod app;
mod route;
mod util;

#[tokio::main]
async fn main() {
    let router = route::init().await;
    let address = "127.0.0.1:9999";
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
