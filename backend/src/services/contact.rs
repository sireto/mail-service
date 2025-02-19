use crate::{models::contact::GetContactResponsee, repositories::contact::{self, ContactRepository, ContactRepositoryImpl}};
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use axum::{http::StatusCode, Json};
use crate::models::contact::{
    Contact,
    CreateContactRequest,
    CreateContactResponse, GetContactResponse, UpdateContactRequest, UpdateContactResponse,
    DeleteContactResponse, ImportResult
};

use super::list_service;

pub struct ContactService {
    repository: Arc<dyn ContactRepository + Send + Sync>
}

impl ContactService {
    pub fn new(repository: Arc<dyn ContactRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_contacts(&self, payload: Vec<CreateContactRequest>) -> Result<Vec<Contact>, diesel::result::Error> {
        self.repository.create_contacts(payload).await
    }

    pub async fn get_all_contacts(&self) -> Result<Vec<Contact>, diesel::result::Error> {
        self.repository.get_all_contacts().await
    }

    pub async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        self.repository.get_contact_by_id(contact_id).await
    }

    pub async fn get_contact_by_email(&self, email: String) -> Result<Contact, diesel::result::Error> {
        self.repository.get_contact_by_email(email).await
    }

    pub async fn update_contact(&self, contact_id: Uuid,
        payload: UpdateContactRequest
    ) -> Result<Contact, diesel::result::Error> {
        self.repository.update_contact(contact_id, payload).await
    }

    pub async fn delete_contact (
        &self,
        contact_id: Uuid,
    ) -> Result<Contact, diesel::result::Error> {
        self.repository.delete_contact(contact_id).await
    }
}


pub async fn create_contacts(
    payloads: Vec<CreateContactRequest> // Corrected syntax
) -> Result<Json<Vec<CreateContactResponse>>, (StatusCode, String)> {
    
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let created_contacts = contact_service.create_contacts(payloads).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = created_contacts.into_iter().map(|contact| CreateContactResponse {
        id: contact.id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
    }).collect();

    Ok(Json(response))
}

pub async fn get_all_contacts() -> Result<Vec<GetContactResponsee>, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);

    println!("Getting all contacts"); 

    let contacts = contact_repository.get_all_contacts().await
    .map_err(|e| {
        println!("Error getting contacts: {:?}", e); // Add this
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

        println!("Found {} base contacts", contacts.len()); // Add this
    
    let contact_ids = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let list_ids = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(list_ids).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let list_map: HashMap<Uuid, String> = lists.into_iter()
        .map(|list| (list.id, list.name))
        .collect();

    let mut response = Vec::new();
    for contact in contacts {
        let list_names = list_contacts.iter()
            .filter(|lc| lc.contact_id == contact.id)
            .filter_map(|lc| list_map.get(&lc.list_id))
            .cloned()
            .collect::<Vec<_>>();

        response.push(GetContactResponsee {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            list_names,
        });
    }

    Ok(response)
}

/// function to get the contact by email...
pub async fn get_contact_by_id(contact_id: String) -> Result<GetContactResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&contact_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact ID format".to_string()))?;

    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_id(uuid_id).await;

    let contact = contact.map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    let contact_response = GetContactResponse {
        id: contact.id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
        created_at: contact.created_at,
        updated_at: contact.updated_at,
    };

    Ok(contact_response)
}

/// function to get the contact by email...
pub async fn get_contact_by_email(email: String) -> Result<GetContactResponse, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_email(email).await;

    let contact = contact.map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    let contact_response = GetContactResponse {
        id: contact.id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
        created_at: contact.created_at,
        updated_at: contact.updated_at,
    };

    Ok(contact_response)
}

pub async fn update_contact (
    contact_id: String,
    payload: UpdateContactRequest
) -> Result<UpdateContactResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&contact_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact ID format".to_string()))?;

    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let updated_contact_response = contact_service.update_contact(uuid_id, payload).await;

    let response_contact = match updated_contact_response {
        Ok(contact) => UpdateContactResponse {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            updated_at: contact.updated_at,
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response_contact)
}

pub async fn delete_contact (
    contact_id: String,
) -> Result<DeleteContactResponse, (StatusCode, String)> {
    // Convert 'contact_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&contact_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let deleted_contact_response = contact_service.delete_contact(uuid_id).await;

    let response_contact = match deleted_contact_response {
        Ok(contact) => DeleteContactResponse {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_contact)
}

pub async fn check_email_exists(email: String) -> Result<bool, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let result = contact_service.get_contact_by_email(email).await;

    match result {
        Ok(contact) => Ok(true), 
        Err(_) => Ok(false),     
    }
}


pub async fn get_all_contactss() -> Result<Vec<GetContactResponsee>, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);

    println!("Getting all contacts"); 

    let contacts = contact_repository.get_all_contacts().await
    .map_err(|e| {
        println!("Error getting contacts: {:?}", e); // Add this
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

        println!("Found {} base contacts", contacts.len()); // Add this
    
    let contact_ids = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let list_ids = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(list_ids).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let list_map: HashMap<Uuid, String> = lists.into_iter()
        .map(|list| (list.id, list.name))
        .collect();

    let mut response = Vec::new();
    for contact in contacts {
        let list_names = list_contacts.iter()
            .filter(|lc| lc.contact_id == contact.id)
            .filter_map(|lc| list_map.get(&lc.list_id))
            .cloned()
            .collect::<Vec<_>>();

        response.push(GetContactResponsee {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            list_names,
        });
    }

    Ok(response)
}

pub async fn import_contacts(
    contacts: Vec<CreateContactRequest>,
    list_ids: Vec<Uuid>,
) -> Result<ImportResult, String> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    
    let mut result = ImportResult {
        imported: 0,
        errors: Vec::new(),
    };

    println!("{:?}", contacts);
    println!("{:?}", list_ids);
    
    // Batch create contacts
    let created_contacts = match contact_service.create_contacts(contacts).await {
        Ok(contacts) => {
            result.imported = contacts.len();
            contacts
        },
        Err(e) => {
            return Err(format!("Failed to create contacts: {}", e));
        }
    };
    
    // Associate contacts with lists if any lists were provided
    if !list_ids.is_empty() && !created_contacts.is_empty() {
        let contact_ids: Vec<Uuid> = created_contacts.iter().map(|c| c.id).collect();
        
        for list_id in &list_ids {
            match list_service::add_contacts_to_list(*list_id, contact_ids.clone()).await {
                Ok(_) => {println!("Succesfully added to list")},
                Err(e) => {
                    result.errors.push(format!("Warning: Created contacts but failed to associate with list {}: {}", list_id, e.1));
                }
            }
        }
    }
    
    Ok(result)
}