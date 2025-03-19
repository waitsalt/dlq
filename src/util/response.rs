use axum::response::IntoResponse;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: DeserializeOwned + Serialize> AppResponse<T> {
    pub fn from(message: &str, data: Option<T>) -> Self {
        Self {
            code: 200,
            message: message.to_string(),
            data: data,
        }
    }

    pub fn success(data: Option<T>) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: data,
        }
    }
}

impl<T: DeserializeOwned + Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
