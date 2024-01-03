use axum::{routing::get, Router};
use crate::module::resource::service::subject_service;
use crate::common::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(subject_service::list_posts).post(subject_service::create_post))
        .route("/:id", get(subject_service::edit_post).post(subject_service::update_post))
}