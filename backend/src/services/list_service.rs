use crate::{models::{contact, list::{CreateListRequest, CreateListResponse, DeleteListResponse, List, ListResponse, UpdateListRequest, UpdatedListResponse}, list_contacts::NewContactInList, template::UpdateTemplateRequest}, repositories::{list_contact_repo, list_repo}};

use aws_sdk_sesv2::types::Status;
use chrono::{DateTime, Utc};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_list(payload: CreateListRequest) -> Result<CreateListResponse, (StatusCode, String)>{
    let new_list = CreateListRequest{
        name: payload.name, 
        namespace_id: payload.namespace_id, 
        description: payload.description, 
    };

    let created_list = list_repo::create_list(new_list).await;

    let response_list = match created_list {
        Ok(list) => CreateListResponse {
            id: list.id.to_string(), 
            name: list.name, 
            created_at: DateTime::from_naive_utc_and_offset(list.created_at, Utc),
        }, 
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };
    Ok(response_list)
}

pub async fn get_all_list (namespace_id: Uuid) -> Result<Vec<ListResponse>, (StatusCode, String)>{
    let all_lists = list_repo::get_all_lists(namespace_id).await;

    let response_lists = match all_lists {
        Ok(lists) => lists.into_iter().map(|list| ListResponse {
            id: list.id, 
            name: list.name, 
            namespace_id: list.namespace_id, 
            description: list.description, 
            created_at: list.created_at, 
            updated_at: list.updated_at
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    }; 
    Ok(response_lists)
}

pub async fn get_list_by_id(namespace_id: Uuid, list_id: Uuid) -> Result<ListResponse, (StatusCode, String)> {
    let list = list_repo::get_list_by_id(namespace_id, list_id).await;

    match list {
        Ok(list) => {
            let response_list = ListResponse {
                id: list.id,
                name: list.name,
                namespace_id: list.namespace_id,
                description: list.description,
                created_at: list.created_at,
                updated_at: list.updated_at,
            };
            Ok(response_list)
        }
        Err(err) => {
            Err((StatusCode::NOT_FOUND, err.to_string()))
        }
    }
}


pub async fn update_list(
    namespace_id: String, // Assuming you get the namespace ID as String too
    list_id: String,
    payload: UpdateListRequest,
) -> Result<UpdatedListResponse, (StatusCode, String)> {
    // Convert both 'namespace_id' and 'list_id' (String) to 'Uuid'
    let uuid_namespace_id = Uuid::parse_str(&namespace_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace UUID format".to_string()))?;
    let uuid_list_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid list UUID format".to_string()))?;

    // Call the repository function to update the list
    let updated_list = list_repo::update_list(uuid_namespace_id, uuid_list_id, payload).await;

    // Handle the result from the repository and return a response
    let response_list = match updated_list {
        Ok(list) => UpdatedListResponse {
            id: list.id,
            name: list.name,
            updated_at: list.updated_at,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string())),
    };

    Ok(response_list)
}

pub async fn delete_list (
    namespace_id: String,
    list_id: String,
) -> Result<DeleteListResponse, (StatusCode, String)> {

    let uuid_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let uuid_namespace_id = Uuid::parse_str(&namespace_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let deleted_list = list_repo::delete_list( uuid_namespace_id, uuid_id).await;

    let response_list = match deleted_list {
        Ok(list) => DeleteListResponse {
            id: list.id,
            name: list.name,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_list)

}

pub async fn add_contacts_to_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<Vec<NewContactInList>, (StatusCode, String)> {
    list_contact_repo::add_contacts_to_list(list_id, contact_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn delete_contacts_from_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<usize, (StatusCode, String)> {
    list_contact_repo::delete_contacts_from_list(list_id, contact_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}