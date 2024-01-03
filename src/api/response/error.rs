use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
}

#[derive(Serialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

pub struct AppError(pub StatusCode, pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(ErrorResponse {
                code: self.0.as_u16() as u32,
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
