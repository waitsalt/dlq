use axum::Json;
use redis::Commands;

use crate::{
    model::user::{User, UserAuth, UserClaim, UserRefreshClaim, UserSigninPayload},
    sql,
    util::{
        AppResult,
        auth::{sign, sign_resfresh},
        config::CONFIG,
        database::database_connect,
        error::AppError,
        redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn signin(Json(user_signin_payload): Json<UserSigninPayload>) -> AppResult<UserAuth> {
    let mut con = redis_connect();

    let captcha_image_key = format!(
        "captcha_image_key:{}",
        user_signin_payload.captcha_image_key
    );
    let captcha_image_value: String = con.get_del(captcha_image_key).unwrap();

    if captcha_image_value != user_signin_payload.captcha_image_value {
        return Err(AppError::CaptchaImageValueError);
    }

    let pool = database_connect();

    let user = sql::user::user_info_get_by_name(pool, &user_signin_payload.name)
        .await
        .unwrap();

    if user.password != user_signin_payload.password {
        return Err(AppError::UserPasswordError);
    }
    auth(user)
}

fn auth(user: User) -> AppResult<UserAuth> {
    let mut con = redis_connect();

    let refresh_token_key = format!("refresh_token:{}", user.id);

    let refresh_token: String = match redis::cmd("GET")
        .arg(refresh_token_key.clone())
        .query(&mut con)
    {
        Ok(refresh_token) => refresh_token,
        Err(_) => "".to_string(),
    };

    let access_token = sign(UserClaim::from(user.clone()))?;

    if !refresh_token.is_empty() {
        Ok(AppResponse::success(Some(UserAuth::new(
            access_token,
            refresh_token,
        ))))
    } else {
        let refresh_token_duration = CONFIG.auth.refresh_token_duration;
        let refresh_token = sign_resfresh(UserRefreshClaim::from(user))?;

        let _: () = redis::cmd("SET")
            .arg(refresh_token_key)
            .arg(refresh_token.clone())
            .arg("EX")
            .arg(refresh_token_duration * 3600 * 24)
            .query(&mut con)
            .unwrap();
        Ok(AppResponse::success(Some(UserAuth::new(
            access_token,
            refresh_token,
        ))))
    }
}
