use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::util::config::CONFIG;

#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub password: String,
    pub email: String,
    pub avatar_url: String, // 头像 url
    pub level: i16,         // 0
    pub status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPublic {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub email: String,
    pub avatar_url: String, // 头像 url
    pub level: i16,         // 0
    pub status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSigninPayload {
    pub name: String,
    pub password: String,
    pub captcha_image_key: String,
    pub captcha_image_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSignupPayload {
    pub name: String,
    pub password: String,
    pub email: String,
    pub avatar_url: String,
    pub captcha_email: String,
    pub captcha_image_key: String,
    pub captcha_image_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserClaim {
    pub iat: i64,
    pub exp: i64,
    pub data: UserPublic,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRefreshClaim {
    pub iat: i64,
    pub exp: i64,
    pub data: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuth {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserPublic {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        Self {
            id: user.id,
            name: user.name,
            desc: user.desc,
            email: user.email,
            avatar_url: user.avatar_url,
            level: user.level,
            status: user.status,
            identity: user.identity,
            create_time: user.create_time,
            update_time: user.update_time,
        }
    }
}

impl UserClaim {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.access_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::minutes(duration);
        Self {
            iat: start_time.timestamp_millis(),
            exp: end_time.timestamp_millis(),
            data: UserPublic::from(user),
        }
    }
}

impl UserRefreshClaim {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.refresh_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::days(duration);
        Self {
            iat: start_time.timestamp_millis(),
            exp: end_time.timestamp_millis(),
            data: user.id,
        }
    }
}

impl UserAuth {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token: access_token,
            refresh_token: refresh_token,
        }
    }
}
