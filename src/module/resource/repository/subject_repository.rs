use crate::module::resource::domain::{subject_entity,subject_entity::Entity as SubjectEntity};
use sea_orm::*;
pub struct Query;
pub struct Mutation;
impl Query {
    pub async fn find_subject_by_id(db: &DbConn, id: i32) -> Result<Option<subject_entity::Model>, DbErr> {
        SubjectEntity::find_by_id(id).one(db).await
    }

    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<subject_entity::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = SubjectEntity::find()
            .order_by_asc(subject_entity::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;
        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: subject_entity::Model,
    ) -> Result<subject_entity::ActiveModel, DbErr> {
        subject_entity::ActiveModel {
            id: Set(form_data.id.to_owned()),
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
            .save(db)
            .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: subject_entity::Model,
    ) -> Result<subject_entity::Model, DbErr> {
        let post: subject_entity::ActiveModel = SubjectEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        subject_entity::ActiveModel {
            id: post.id,
            name: Set(form_data.name.to_owned()),
        }
            .update(db)
            .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: subject_entity::ActiveModel = SubjectEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        SubjectEntity::delete_many().exec(db).await
    }
}
