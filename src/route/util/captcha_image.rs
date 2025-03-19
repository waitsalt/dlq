use axum::extract::Path;
use captcha_rust::Captcha;
use redis::Commands;

use crate::util::{AppResult, redis::redis_connect, response::AppResponse};

pub async fn captcha_image(Path(captcha_image_key): Path<String>) -> AppResult<String> {
    let captcha = Captcha::new(5, 100, 40);
    let mut con = redis_connect();
    let captcha_image_key = format!("captcha_image_key:{}", captcha_image_key);
    let _: () = con.set_ex(captcha_image_key, captcha.text, 5 * 60).unwrap();
    Ok(AppResponse::success(Some(captcha.base_img)))
}
