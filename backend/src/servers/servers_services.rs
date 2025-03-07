use crate::{error::AppError, servers::servers_repo::{ServerRepo, ServerRepoImpl}};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::servers::servers_model::{Server, ServerRequest};
use async_trait::async_trait;

#[async_trait]
pub trait ServerServiceTrait {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, AppError>;
    async fn get_all_servers(&self) -> Result<Vec<Server>, AppError>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, AppError>;
    async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, AppError>;
    async fn delete_server(&self, server_id: &str) -> Result<Server, AppError>;
}
#[derive(Clone)]
pub struct ServerService {
    repository: Arc<dyn ServerRepo + Send + Sync>
}

impl ServerService {
    pub fn new(repository: Arc<dyn ServerRepo + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ServerServiceTrait for ServerService {

     async fn create_server(&self, payload: ServerRequest) -> Result<Server, AppError> {
        let server = self.repository.create_server(payload).await?;

        Ok(server)
    }

     async fn get_all_servers(&self) -> Result<Vec<Server>, AppError> {
        let servers = self.repository.get_all_servers().await?;

        Ok(servers)
    }

     async fn get_server_by_id(&self, server_id: &str) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let server = self.repository.get_server_by_id(uuid_id).await?;

        Ok(server)
    }

     async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let updated_server = self.repository.update_server(uuid_id, payload).await?;

        Ok(updated_server)
    }

     async fn delete_server(&self, server_id: &str) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let deleted_server = self.repository.delete_server(uuid_id).await?;

        Ok(deleted_server)
    }
}
