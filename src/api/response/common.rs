use std::borrow::Cow;

use axum::{response::IntoResponse, Json};

#[derive(serde::Serialize)]
pub struct WebApiResponse<T>
{
  pub code: u16,
  pub message: Cow<'static, str>,
  pub data: T,
}

impl<T> WebApiResponse<T>
{
  pub fn new(message: Cow<'static, str>, data: T) -> Self {
    Self {
      code: 0,
      message,
      data,
    }
  }
}

impl<T> IntoResponse for WebApiResponse<T>
where
  T: serde::Serialize,
{
  fn into_response(self) -> axum::response::Response {
    Json(self).into_response()
  }
}

impl<T> From<T> for WebApiResponse<T>
where
  T: serde::Serialize,
{
  fn from(data: T) -> Self {
    Self::new("success".into(), data)
  }
}

impl WebApiResponse<()> {
  pub fn from_str(message: Cow<'static, str>) -> Self {
    Self::new(message, ())
  }
}
#[derive(serde::Serialize)]
pub struct EmptyResponse{
}
impl<T: Default> Default for WebApiResponse<T> {
  fn default() -> Self {
    Self {
      code: 0,
      message: Cow::from(""),
      data: T::default(),
    }
  }
}
