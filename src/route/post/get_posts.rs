use axum::extract::Path;

pub async fn get_posts(
    Path(page): Path<u16>,
    Path(limit): Path<u16>,
    Path(keyword): Path<String>,
) -> () {
}
