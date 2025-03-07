
use crate::models::{list::{CreateListRequest, CreateListResponse, DeleteListResponse, ListResponse, UpdateListRequest, UpdatedListResponse}, list_contacts::{NewContactInList, AddContactRequest}};
use axum::{
    extract:: Path, Json
};
use uuid::Uuid;
use crate::error::AppError;
use crate::services::list_service;



#[utoipa::path(
    post, 
    path = "/api/list", 
    responses(
        (status=200, description = "Create a new list", body=CreateListResponse), 
        (status = 404)
    )
)]
pub async fn create_list(
    Json(payload): Json<CreateListRequest>,
) ->Result<Json<CreateListResponse>, AppError> {
    
    let create_new_list = list_service::create_list(payload).await?;

    let create_response = CreateListResponse {
        id: create_new_list.id.to_string(), 
        name: create_new_list.name, 
        created_at: create_new_list.created_at
    };

    Ok(Json(create_response))
}

#[utoipa::path(
    get, 
    path = "/api/list/namespaces/{namespace_id}/list", 
    responses(
        (status=200, description="List of lists", body=Vec<ListResponse>), 
        (status= 404)
    )
)]
pub async fn get_lists(Path(namespace_id): Path<String>) -> Result<Json<Vec<ListResponse>>, AppError> {
    let namespace_id = Uuid::parse_str(&namespace_id)?;
    let lists_result = list_service::get_all_list(namespace_id).await?;

    if lists_result.is_empty() {
        return Ok(Json(vec![]));
    }
    Ok(Json(lists_result))
}


#[utoipa::path(
    get, 
    path = "/api/list/namespaces/{namespace_id}/list/{list_id}", 
    responses(
        (status = 200, description = "Get List By ID", body=ListResponse), 
        (status =404)
    )
)]
pub async fn get_list_by_id(
    Path((namespace_id, list_id)): Path<(String, String)>, // Extract both namespace_id and list_id as Strings
) -> Result<Json<ListResponse>, AppError> {
    // Parse namespace_id and list_id to Uuid
    let namespace_id = Uuid::parse_str(&namespace_id)?;
    
    let list_id = Uuid::parse_str(&list_id)?;

    // Call service layer with parsed Uuids
    let list_result = list_service::get_list_by_id(namespace_id, list_id).await?;

    Ok(Json(list_result))
}

#[utoipa::path(
    patch, 
    path = "/api/list/namespaces/{namespace_id}/list/{list_id}", 
    params(
        ("list_id", Path, description ="Id of the lsit to update")
    ), 
    responses(
        (status = 200, description = "List updated successfully", body = UpdatedListResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_list(
    Path((namespace_id, list_id)): Path<(String, String)>, // Extract both namespace_id and list_id as Strings
    Json(payload): Json<UpdateListRequest>,
) -> Result<Json<UpdatedListResponse>, AppError> {
    let uuid_namespace_id = Uuid::parse_str(&namespace_id)?;
    let uuid_list_id = Uuid::parse_str(&list_id)?;
    // Call the service function with the UUIDs
    let update_list_response = list_service::update_list(uuid_namespace_id, uuid_list_id, payload).await?;

    // Return the updated list response
    Ok(Json(update_list_response))
}

#[utoipa::path(
    delete,
    path = "/api/list/namespaces/{namespace_id}/list/{list_id}",
    params(
        ("list_id" = String, Path, description = "ID of the list to delete")
    ),
    responses(
        (status = 200, description = "List deleted successfully", body = DeleteListResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "List not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_list(Path((namespace_id, list_id)): Path<(String, String)>) -> Result<Json<DeleteListResponse>, AppError> {
    let uuid_namespace_id = Uuid::parse_str(&namespace_id)?;
    let uuid_list_id = Uuid::parse_str(&list_id)?;
    let delete_list_response = list_service::delete_list(uuid_namespace_id, uuid_list_id).await?;

    Ok(Json(delete_list_response))
}

#[utoipa::path(
    post,
    path = "/api/list/addContacts/{list_id}",
    params(
        ("list_id" = String, Path, description = "ID of the list where contact is to be added")
    ),
    responses(
        (status = 200, description = "List deleted successfully", body = DeleteListResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "List not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_contacts_to_list(
    Path(list_id): Path<String>,
    Json(payload): Json<AddContactRequest>,
) -> Result<Json<Vec<NewContactInList>>, AppError> {
    let list_id = Uuid::parse_str(&list_id)?;

    let added_contacts = list_service::add_contacts_to_list(list_id, payload.contact_ids)
        .await?;

    Ok(Json(added_contacts))
}


#[utoipa::path(
    delete,
    path = "/api/list/removeContacts/{list_id}",
    params(
        ("list_id" = String, Path, description = "ID of the list to remove contacts from")
    ),
    request_body = AddContactRequest,
    responses(
        (status = 200, description = "Contacts removed successfully", body = usize),
        (status = 400, description = "Bad request"),
        (status = 404, description = "List not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn remove_contacts_from_list(
    Path(list_id): Path<String>,
    Json(payload): Json<AddContactRequest>,
) -> Result<Json<usize>, AppError> {
    let list_id = Uuid::parse_str(&list_id)?;

    let num_deleted = list_service::delete_contacts_from_list(list_id, payload.contact_ids)
        .await?;

    Ok(Json(num_deleted))
}