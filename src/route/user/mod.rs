mod signin;
mod signout;
mod signup;

use axum::{
    Router,
    routing::{get, post},
};

pub fn init() -> Router {
    Router::new()
        .route("/signin", post(signin::signin))
        .route("/signout", get(signout::signout))
        .route("/signup", post(signup::signup))
}
