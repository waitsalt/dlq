mod model;
mod route;
mod sql;
mod util;

use util::config::CONFIG;

#[tokio::main]
async fn main() {
    // 初始化项目功能模块
    util::init().await;

    let address = format!("127.0.0.1:{}", CONFIG.server.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("address bind error");
    let router = route::init();
    tracing::info!("Server start: http://{}", address);
    axum::serve(listener, router).await.expect("app run error")
}
