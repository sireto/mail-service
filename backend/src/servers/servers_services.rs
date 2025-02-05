use crate::servers::servers_repo::{ServerRepo, ServerRepoImpl};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::servers::servers_model::{Server, ServerRequest};
use async_trait::async_trait;

#[async_trait]
pub trait ServerServiceTrait {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, (StatusCode, String)>;
    async fn get_all_servers(&self) -> Result<Vec<Server>, (StatusCode, String)>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, (StatusCode, String)>;
    async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, (StatusCode, String)>;
    async fn delete_server(&self, server_id: &str) -> Result<Server, (StatusCode, String)>;
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

     async fn create_server(&self, payload: ServerRequest) -> Result<Server, (StatusCode, String)> {
        let server = self.repository.create_server(payload).await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(server)
    }

     async fn get_all_servers(&self) -> Result<Vec<Server>, (StatusCode, String)> {
        let servers = self.repository.get_all_servers().await
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

        Ok(servers)
    }

     async fn get_server_by_id(&self, server_id: &str) -> Result<Server, (StatusCode, String)> {
        let uuid_id = Uuid::parse_str(server_id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid server ID format".to_string()))?;

        let server = self.repository.get_server_by_id(uuid_id).await
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

        Ok(server)
    }

     async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, (StatusCode, String)> {
        let uuid_id = Uuid::parse_str(server_id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid server ID format".to_string()))?;

        let updated_server = self.repository.update_server(uuid_id, payload).await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(updated_server)
    }

     async fn delete_server(&self, server_id: &str) -> Result<Server, (StatusCode, String)> {
        let uuid_id = Uuid::parse_str(server_id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid server ID format".to_string()))?;

        let deleted_server = self.repository.delete_server(uuid_id).await
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

        Ok(deleted_server)
    }
}
