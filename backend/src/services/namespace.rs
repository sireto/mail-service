use crate::{models::namespace::GetNamespaceResponse, repositories::namespace::{self, NamespaceRepository, NamespaceRepositoryImpl}};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::namespace::{
    Namespace,
    CreateNamespaceRequest,
    CreateNamespaceResponse,
    UpdateNamespaceRequest,
    UpdateNamespaceResponse,
    DeleteNamespaceResponse
};

#[derive(Clone)]
pub struct NamespaceService {
    repository: Arc<dyn NamespaceRepository + Send + Sync>
}

impl NamespaceService {
    pub fn new(repository: Arc<dyn NamespaceRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_namespace(&self, payload: CreateNamespaceRequest) -> Result<CreateNamespaceResponse, (StatusCode, String)> {
        let response = self.repository.create_namespace(payload).await;

        let response = match response {
            Ok(namespace) => CreateNamespaceResponse {
                id: namespace.id,
                name: namespace.name,
                created_at: namespace.created_at,
            },
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        };

        Ok(response)
    }

    pub async fn get_all_namespaces(&self) -> Result<Vec<GetNamespaceResponse>, (StatusCode, String)> {
        let response = self.repository.get_all_namespaces().await;

        let response = match response {
            Ok(namespaces) => namespaces.into_iter().map(|namespace| GetNamespaceResponse {
                id: namespace.id,
                name: namespace.name,
                created_at: namespace.created_at,
                updated_at: namespace.updated_at
            }).collect(),
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        };

        Ok(response)
    }

    pub async fn get_namespace_by_id(&self, namespace_id: Uuid) -> Result<GetNamespaceResponse, (StatusCode, String)> {
        let response = self.repository.get_namespace_by_id(namespace_id).await;

        let response = match response {
            Ok(namespace) => GetNamespaceResponse {
                id: namespace.id,
                name: namespace.name,
                created_at: namespace.created_at,
                updated_at: namespace.updated_at
            },
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        };

        Ok(response)
    }

    pub async fn update_namespace(&self, namespace_id: Uuid, payload: UpdateNamespaceRequest) -> Result<UpdateNamespaceResponse, (StatusCode, String)> {
        let response = self.repository.update_namespace(namespace_id, payload).await;

        let response = match response {
            Ok(namespace) => UpdateNamespaceResponse {
                id: namespace.id,
                name: namespace.name,
                updated_at: namespace.updated_at,
            },
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        };

        Ok(response)
    }

    pub async fn delete_namespace(&self, namespace_id: Uuid) -> Result<DeleteNamespaceResponse, (StatusCode, String)> {
        let response = self.repository.delete_namespace(namespace_id).await;

        let response = match response {
            Ok(namespace) => DeleteNamespaceResponse {
                id: namespace.id,
                name: namespace.name
            },
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        };

        Ok(response)
    }
}