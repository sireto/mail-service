use crate::{models::contact::
    { 
        Contact, CreateContactRequest, CreateContactResponse, DeleteContactResponse, EmailQuery, GetContactResponse, GetContactResponsee, UpdateContactRequest, UpdateContactResponse, ImportOptions, ImportResponse
    }, services::contact
};
use crate::services::contact as contact_service;

use axum::{
    extract::{ Path, Query}, http::StatusCode, Json
};

use axum_extra::extract::Multipart;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::utils::contact_lists_functions::parse_csv_data;

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
pub async fn get_contacts() -> Result<Json<Vec<GetContactResponsee>>, (StatusCode, String)> {
    println!("Handler called");
    let contacts = contact_service::get_all_contactss().await?;

    if contacts.is_empty() {
        println!("No contacts found");
        return Err((StatusCode::NOT_FOUND, "No contacts found".to_string()));
    }

    println!("Found {} contacts", contacts.len()); 
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


#[utoipa::path(
    post,
    path = "/api/contacts/import",
    request_body = ImportOptions,
    responses(
        (status = 200, description = "Import contacts from CSV", body = ImportResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn import_contacts(
    options: Query<ImportOptions>,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, (StatusCode, String)> {
    // Extract file from multipart form
    let mut file_data = Vec::new();
    let mut file_found = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to process multipart form: {}", e),
        )
    })? {
        if field.name() == Some("file") {
            file_data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read file data: {}", e),
                )
            })?
            .to_vec();
            file_found = true;
            break;
        }
    }

    if !file_found {
        return Err((StatusCode::BAD_REQUEST, "No file found in the request".to_string()));
    }

    // Parse list ids from the options
    let list_ids = match &options.lists {
        Some(lists_json) => {
            serde_json::from_str::<Vec<String>>(lists_json)
                .map(|strs| {
                    strs.iter()
                        .filter_map(|s| Uuid::parse_str(s).ok())
                        .collect::<Vec<Uuid>>()
                })
                .unwrap_or_else(|_| Vec::new())
        }
        None => Vec::new(),
    };

    // Parse the csv file
    let contacts = parse_csv_data(
        &file_data,
        options.delimiter.as_deref().unwrap_or(","),
        options.mode.as_deref().unwrap_or("subscribe"),
        options.status.as_deref().unwrap_or("unconfirmed"),
        options.overwrite.unwrap_or(false),
    )
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to parse CSV: {}", e)))?;

    // Import contacts
    let result = contact_service::import_contacts(contacts, list_ids).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to import contacts: {}", e)))?;

    Ok(Json(ImportResponse {
        success: true,
        imported: result.imported,
        errors: if result.errors.is_empty() {
            None
        } else {
            Some(result.errors)
        },
    }))
}

