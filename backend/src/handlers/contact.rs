use crate::{models::contact::
    { 
        Contact, CreateContactRequest, CreateContactResponse, DeleteContactResponse, EmailQuery, GetContactResponse, UpdateContactRequest, UpdateContactResponse
    }, services::contact
};
use crate::services::contact as contact_service;

use axum::{
    extract::{ Path, Query}, http::StatusCode, Json
};

#[utoipa::path(
    post,
    path = "/api/contacts",
    responses(
        (status = 200, description = "Create a contact", body = CreateContactRequest),
        (status = 404)
    )
)]
pub async fn create_contacts(
    Json(payloads): Json<Vec<CreateContactRequest>>, // Corrected JSON extractor
) -> Result<Json<Vec<CreateContactResponse>>, (StatusCode, String)> {
    
    let created_contacts = contact_service::create_contacts(payloads).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "NO Contacts Created".to_string()))?; 

    let response: Vec<CreateContactResponse> = created_contacts.iter().map(|contact| CreateContactResponse {
        id: contact.id,
        first_name: contact.first_name.clone(),  
        last_name: contact.last_name.clone(),    
        email: contact.email.clone(),            
        attribute: contact.attribute.clone(),    
    }).collect();

    Ok(Json(response)) // Wrap response inside Json()
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


#[utoipa::path(
    get,
    path = "/api/contacts/check-email",
    params(
        EmailQuery
    ),
    responses(
        (status = 200, description = "Email existence check result", body = bool),
        (status = 400, description = "Invalid email format"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn check_email(
    Query(query): Query<EmailQuery>
) -> Result<Json<bool>, (StatusCode, String)> {

    match contact::check_email_exists(query.email).await {
        Ok(exists) => Ok(Json(exists)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR, 
            format!("{:?}", err)  
        )),
    }
}