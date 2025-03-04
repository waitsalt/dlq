mod app;
mod model;
mod route;
mod util;

use util::config::CONFIG;

#[tokio::main]
async fn main() {
    let app = app::init();
    let address = format!("127.0.0.1:{}", CONFIG.server.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("address bind fail");
    tracing::info!("server run in http://{}", address);
    axum::serve(listener, app).await.unwrap();
}
