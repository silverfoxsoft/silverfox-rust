use crate::module::resource::repository::subject_repository::{
    Mutation as MutationCore, Query as QueryCore,
};
use crate::module::resource::domain::subject_entity;
use serde::{Deserialize, Serialize};
use axum::{extract::{Form, Path, Query, State}, http::StatusCode};
use crate::api::response::common::WebApiResponse;
use crate::api::response::error::AppError;
use crate::module::resource::domain::subject_entity::Model;
use crate::common::app_state::AppState;

#[derive(Deserialize)]
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}
pub async fn edit_post(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<WebApiResponse<()>, AppError> {
    let post: Model = QueryCore::find_subject_by_id(&state.conn, id)
        .await
        .expect("could not find post")
        .unwrap_or_else(|| panic!("could not find post with id {id}"));

    Ok(WebApiResponse::<()>::from_str("执行成功".into()))
}
pub async fn list_posts(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<WebApiResponse<PageResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(5);
    let (posts, num_pages) = QueryCore::find_posts_in_page(&state.conn, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");
    let page_response = PageResponse{
        list: posts,
        page_num: page as i32,
        total_page: num_pages as i32,
    };
    Ok(WebApiResponse::<PageResponse>::from(page_response))
}
#[derive(Serialize, Debug)]
pub struct PageResponse{
    pub list: Vec<Model>,
    pub page_num: i32,
    pub total_page: i32,

}
pub async fn create_post(
    state: State<AppState>,
    form: Form<subject_entity::Model>,
) -> Result<WebApiResponse<()>, AppError> {
    let form = form.0;

    MutationCore::create_post(&state.conn, form)
        .await
        .expect("could not insert post");

    Ok(WebApiResponse::<()>::from_str("执行成功".into()))
}
pub async fn update_post(
    state: State<AppState>,
    Path(id): Path<i32>,
    form: Form<Model>,
) -> Result<WebApiResponse::<()>, (StatusCode, String)> {
    let form = form.0;

    MutationCore::update_post_by_id(&state.conn, id, form)
        .await
        .expect("could not edit post");
    Ok(WebApiResponse::<()>::from_str("执行成功".into()))
}

