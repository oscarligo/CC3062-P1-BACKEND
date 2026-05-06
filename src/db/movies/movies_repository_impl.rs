use async_trait::async_trait;
use sea_orm::*;
use crate::models::movie::movie::{Entity as Movie, Model, ActiveModel};

use super::movies_repository::MovieRepository;

pub struct SeaOrmMovieRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl MovieRepository for SeaOrmMovieRepository {
    async fn get_all(&self) -> Result<Vec<Model>, DbErr> {
        Movie::find().all(&self.db).await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Movie::find_by_id(id).one(&self.db).await
    }

    async fn create(&self, data: Model) -> Result<Model, DbErr> {
        let active_model: ActiveModel = data.into_active_model();
        active_model.insert(&self.db).await
    }

    async fn update(&self, id: i32, data: Model) -> Result<Model, DbErr> {
        let movie_to_update = Movie::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Movie not found".to_owned()))?;

        let mut active_model: ActiveModel = movie_to_update.into();
        active_model.title = Set(data.title);
        active_model.poster = Set(data.poster);
        
        active_model.update(&self.db).await
    }

    async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let res = Movie::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected)
    }
}