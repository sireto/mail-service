use crate::models::{list::{CreateListRequest, CreateListResponse, DeleteListResponse, ListResponse, UpdateListRequest, UpdatedListResponse}};

use axum::{
    extract:: Path, Json, http::StatusCode
};
use uuid::Uuid;

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
) ->Result<Json<CreateListResponse>, (StatusCode, String)> {
    
    let create_new_list = list_service::create_list_service(payload).await?;

    let create_response = CreateListResponse {
        id: create_new_list.id.to_string(), 
        name: create_new_list.name, 
        created_at: create_new_list.created_at
    };

    Ok(Json(create_response))
}

#[utoipa::path(
    get, 
    path = "/api/list", 
    responses(
        (status=200, description="List of lists", body=Vec<ListResponse>), 
        (status= 404)
    )
)]
pub async fn get_lists() -> Result<Json<Vec<ListResponse>>, (StatusCode, String)> {
    let lists_result = list_service::get_all_list_services().await?;

    if lists_result.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No Lists found".to_string()));
    }
    Ok(Json(lists_result))
}


#[utoipa::path(
    get, 
    path = "/api/list/{list_id}", 
    responses(
        (status = 200, description = "Get List By ID", body=ListResponse), 
        (status =404)
    )
)]
pub async fn get_list_by_id(Path(list_id): Path<String>) -> Result<Json<ListResponse>, (StatusCode, String)> {
    let list_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid list ID format".to_string()))?;
    
    let list_result = list_service::get_list_by_id_service(list_id).await;

    match list_result {
        Ok(list) => Ok(Json(list)),
        Err((status, message))=>Err((status, message)),
    }
}


#[utoipa::path(
    patch, 
    path = "api/list/{list_id}", 
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
pub async fn update_list(Path(list_id): Path<String>, Json(payload): Json<UpdateListRequest>)-> Result<Json<UpdatedListResponse>, (StatusCode, String)>{
    
    let update_list_response = list_service::update_list_service(list_id, payload).await?;

    Ok(Json(update_list_response))
}

#[utoipa::path(
    delete,
    path = "/api/lists/{list_id}",
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
pub async fn delete_list(Path(list_id): Path<String>) -> Result<Json<DeleteListResponse>, (StatusCode, String)> {
    let delete_list_response = list_service::delete_list_service(list_id).await?;

    Ok(Json(delete_list_response))
}