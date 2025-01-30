use crate::{models::contact::
    { 
        Contact,
        CreateContactRequest, 
        CreateContactResponse, 
        GetContactResponse, 
        UpdateContactRequest,
        UpdateContactResponse,
        DeleteContactResponse
    }, services::contact
};
use crate::services::contact as contact_service;

use axum::{
    extract:: Path, Json, http::StatusCode
};

#[utoipa::path(
    post,
    path = "/api/contacts",
    responses(
        (status = 200, description = "Create a contact", body = CreateContactRequest),
        (status = 404)
    )
)]
pub async fn create_contact(
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<CreateContactResponse>, (StatusCode, String)> {

    let created_contact = contact::create_contact(payload).await?;

    Ok(Json(created_contact))
}

#[utoipa::path(
    get,
    path = "/api/contacts",
    responses(
        (status = 200, description = "Get all the contacts", body = Vec<GetContactResponse>),
        (status = 404)
    )
)]
pub async fn get_contacts() -> Result<Json<Vec<GetContactResponse>>, (StatusCode, String)> {
    let contacts = contact_service::get_all_contacts().await?;

    if contacts.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No contacts found".to_string()));
    }

    Ok(Json(contacts))
}

#[utoipa::path(
    get,
    path = "/api/contacts/{contact_id}",
    responses(
        (status = 200, description = "Get a contact by id", body = GetContactResponse),
        (status = 404)
    )
)]
pub async fn get_contact_by_id(Path(contact_id): Path<String>) -> Result<Json<GetContactResponse>, (StatusCode, String)> {
    let contact = contact_service::get_contact_by_id(contact_id).await?;

    Ok(Json(contact))
}

#[utoipa::path(
    patch,
    path = "/api/contacts/{contact_id}",
    params(
        ("contact_id" = String, Path, description = "ID of the contact to update")
    ),
    responses(
        (status = 200, description = "Contact updated successfully", body = UpdateContactResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Contact not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_contact(
    
    Path(contact_id): Path<String>,
    Json(payload): Json<UpdateContactRequest>
) -> Result<Json<UpdateContactResponse>, (StatusCode, String)> {

    let update_contact_response = contact_service::update_contact(contact_id, payload).await?;

    Ok(Json(update_contact_response))
}


#[utoipa::path(
    delete,
    path = "/api/contacts/{contact_id}",
    params(
        ("contact_id" = String, Path, description = "ID of the contact to delete")
    ),
    responses(
        (status = 200, description = "Contact deleted successfully", body = DeleteContactResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Contact not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_contact(
    Path(contact_id): Path<String>
) -> Result<Json<DeleteContactResponse>, (StatusCode, String)> {
    let delete_contact_response = contact::delete_contact(contact_id).await?;

    Ok(Json(delete_contact_response))
}
