use crate::{models::{list::{CreateListRequest, CreateListResponse, DeleteListResponse, List, ListResponse, UpdateListRequest, UpdatedListResponse}, template::UpdateTemplateRequest}, repositories::list_repo};

use aws_sdk_sesv2::types::Status;
use chrono::{DateTime, Utc};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_list_service(payload: CreateListRequest) -> Result<CreateListResponse, (StatusCode, String)>{
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

pub async fn get_all_list_services () -> Result<Vec<ListResponse>, (StatusCode, String)>{
    let all_lists = list_repo::get_all_lists().await;

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

pub async fn get_list_by_id_service(list_id: Uuid) -> Result<ListResponse, (StatusCode, String)> {
    let list = list_repo::get_list_by_id(list_id).await;

    match list {
        Ok(list) =>{
            let response_list =ListResponse {
                id: list.id, 
                name: list.name, 
                namespace_id: list.namespace_id, 
                description: list.description, 
                created_at: list.created_at, 
                updated_at: list.updated_at
            };
            Ok(response_list)
        }, 
        Err(err) => {
            Err((StatusCode::NOT_FOUND, err.to_string()))
        }
    }

}


pub async fn update_list_service (
    list_id: String,
    payload: UpdateListRequest
) -> Result<UpdatedListResponse, (StatusCode, String)> {
    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let updated_list = list_repo::update_list( uuid_id, payload).await;

    let response_list = match updated_list {
        Ok(list) => UpdatedListResponse {
            id: list.id,
            name: list.name,
            updated_at: list.updated_at,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_list)
}

pub async fn delete_list_service (
    list_id: String,
) -> Result<DeleteListResponse, (StatusCode, String)> {

    let uuid_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let deleted_list = list_repo::delete_list( uuid_id).await;

    let response_list = match deleted_list {
        Ok(list) => DeleteListResponse {
            id: list.id,
            name: list.name,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_list)
}