use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // user
    UserNameExist,
    UserEmailExist,
    UserPasswordShort,
    UserNotExist,
    UserBlocked,
    UserDeleted,
    UserMissPermission,
    UserCreateFailure,
    UserPasswordError,

    // auth
    AccessTokenMiss,
    AccessTokenInvalid,
    RefreshTokenMiss,
    RefreshTokenInvalid,
    CaptchaImageError,
    CaptchaEmailError,

    // sqlx
    SqlxError,

    // notfount
    NotFound,

    // other
    Other,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<sqlx::error::Error> for AppError {
    fn from(_: sqlx::error::Error) -> Self {
        AppError::SqlxError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let bad_request = StatusCode::OK;
        let (status_code, code, message) = match self {
            // user
            AppError::UserNameExist => (bad_request, 1001, "user name exist"),
            AppError::UserEmailExist => (bad_request, 1002, "user email exist"),
            AppError::UserPasswordShort => (bad_request, 1002, "user password too short"),
            AppError::UserNotExist => (bad_request, 1003, "用户不存在"),
            AppError::UserBlocked => (bad_request, 1004, "用户被封禁"),
            AppError::UserDeleted => (bad_request, 1005, "用户已被删除"),
            AppError::UserMissPermission => (bad_request, 1006, "用户没有权限"),
            AppError::UserCreateFailure => (bad_request, 1007, "用户创建失败"),
            AppError::UserPasswordError => (bad_request, 1008, "用户密码错误"),

            // auth
            AppError::AccessTokenMiss => (bad_request, 2001, "access token miss"),
            AppError::AccessTokenInvalid => (bad_request, 2002, "access token invalid"),
            AppError::RefreshTokenMiss => (bad_request, 2003, "refresh token miss"),
            AppError::RefreshTokenInvalid => (bad_request, 2004, "refresh token invalid"),
            AppError::CaptchaEmailError => (bad_request, 2005, "邮箱验证码错误"),
            AppError::CaptchaImageError => (bad_request, 2006, "图形验证码错误"),

            // sqlx
            AppError::SqlxError => (bad_request, 3001, "sql 出现问题"),

            // not found
            AppError::NotFound => (bad_request, 4004, "not found"),

            // Other
            AppError::Other => (bad_request, 5000, "not known wrong"),
        };
        let body = Json(json!({
            "code": code,
            "message": message,
            "data": (),
        }));
        (status_code, body).into_response()
    }
}
