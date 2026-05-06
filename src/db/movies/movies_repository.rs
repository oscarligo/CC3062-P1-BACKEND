use async_trait::async_trait;
use crate::models::movie; 
use sea_orm::DbErr;

#[async_trait]
pub trait MovieRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<movie::Model>, DbErr>;
    async fn get_by_id(&self, id: i32) -> Result<Option<movie::Model>, DbErr>;
    async fn create(&self, data: movie::Model) -> Result<movie::Model, DbErr>;
    async fn update(&self, id: i32, data: movie::Model) -> Result<movie::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<u64, DbErr>;
}