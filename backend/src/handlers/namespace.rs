use std::sync::Arc;
use crate::models::namespace::
    { 
        CreateNamespaceRequest, CreateNamespaceResponse, DeleteNamespaceResponse, GetNamespaceResponse, Namespace, UpdateNamespaceRequest, UpdateNamespaceResponse
    };
use crate::services::namespace::{self as namespace_service, NamespaceService};
use crate::repositories::namespace::{
    NamespaceRepository,
    NamespaceRepositoryImpl
};

use axum::{
    extract:: Path, Json, http::StatusCode, extract::State
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/namespaces",
    responses(
        (status = 200, description = "Create a namespace", body = CreateNamespaceRequest),
        (status = 404)
    )
)]
pub async fn create_namespace(
    State(namespace_service): State<Arc<NamespaceService>>,
    Json(payload): Json<CreateNamespaceRequest>,
) -> Result<Json<CreateNamespaceResponse>, (StatusCode, String)> {
    let created_namespace = namespace_service.create_namespace(payload).await?;

    Ok(Json(created_namespace))
}

#[utoipa::path(
    get,
    path = "/api/namespaces",
    responses(
        (status = 200, description = "Get all the namespaces", body = Vec<GetNamespaceResponse>),
        (status = 404)
    )
)]
pub async fn get_all_namespaces(
    State(namespace_service): State<Arc<NamespaceService>>
) -> Result<Json<Vec<GetNamespaceResponse>>, (StatusCode, String)> {
    let all_namespaces = namespace_service.get_all_namespaces().await?;

    Ok(Json(all_namespaces))
}

#[utoipa::path(
    get,
    path = "/api/namespaces/{namespace_id}",
    params(
        ("namespace_id" = String, Path, description = "ID of the namespace to get")
    ),
    responses(
        (status = 200, description = "Get a namespace by ID", body = GetNamespaceResponse),
        (status = 404)
    )
)]
pub async fn get_namespace_by_id(
    State(namespace_service): State<Arc<NamespaceService>>,
    Path(namespace_id): Path<String>,
) -> Result<Json<GetNamespaceResponse>, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&namespace_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace ID format".to_string()))?;

    let namespace = namespace_service.get_namespace_by_id(uuid_id).await?;

    Ok(Json(namespace))
}

#[utoipa::path(
    patch,
    path = "/api/namespaces/{namespace_id}",
    params(
        ("namespace_id" = String, Path, description = "ID of the namespace to update")
    ),
    responses(
        (status = 200, description = "Update a namespace", body = CreateNamespaceRequest),
        (status = 404)
    )
)]
pub async fn update_namespace(
    State(namespace_service): State<Arc<NamespaceService>>,
    Path(namespace_id): Path<String>,
    Json(payload): Json<UpdateNamespaceRequest>,
) -> Result<Json<UpdateNamespaceResponse>, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&namespace_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace ID format".to_string()))?;

    let updated_namespace = namespace_service.update_namespace(uuid_id, payload).await?;

    Ok(Json(updated_namespace))
}

#[utoipa::path(
    delete,
    path = "/api/namespaces/{namespace_id}",
    params(
        ("namespace_id" = String, Path, description = "ID of the namespace to delete")
    ),
    responses(
        (status = 200, description = "Delete a namespace", body = CreateNamespaceRequest),
        (status = 404)
    )
)]
pub async fn delete_namespace(
    State(namespace_service): State<Arc<NamespaceService>>,
    Path(namespace_id): Path<String>,
) -> Result<Json<DeleteNamespaceResponse>, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&namespace_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace ID format".to_string()))?;

    let deleted_namespace = namespace_service.delete_namespace(uuid_id).await?;

    Ok(Json(deleted_namespace))
}