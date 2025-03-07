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
use crate::error::AppError;

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
) -> Result<Vec<CreateContactResponse>, AppError> {
    
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let created_contacts = contact_service.create_contacts(payloads).await?;

    let response = created_contacts.into_iter().map(|contact| CreateContactResponse {
        id: contact.id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
    }).collect();

    Ok(response)
}

pub async fn get_all_contacts() -> Result<Vec<GetContactResponsee>, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);

    let contacts = contact_repository.get_all_contacts().await?;

        println!("Found {} base contacts", contacts.len()); // Add this
    
    let contact_ids = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids).await?;
    
    let list_ids = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(list_ids).await?;
    
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
pub async fn get_contact_by_id(contact_id: Uuid) -> Result<GetContactResponse, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_id(contact_id).await;

    let contact = contact.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

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
pub async fn get_contact_by_email(email: String) -> Result<GetContactResponse, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_email(email).await;

    let contact = contact.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

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
    contact_id: Uuid,
    payload: UpdateContactRequest
) -> Result<UpdateContactResponse, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let updated_contact_response = contact_service.update_contact(contact_id, payload).await?;

    Ok(UpdateContactResponse {
        id: updated_contact_response.id,
        first_name: updated_contact_response.first_name,
        last_name: updated_contact_response.last_name,
        email: updated_contact_response.email,
        attribute: updated_contact_response.attribute,
        updated_at: updated_contact_response.updated_at,
    })
}

pub async fn delete_contact (
    contact_id: Uuid,
) -> Result<DeleteContactResponse, AppError> {
    // Convert 'contact_id' (String) to 'Uuid'...
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let deleted_contact_response = contact_service.delete_contact(contact_id).await?;

    Ok(DeleteContactResponse {
        id: deleted_contact_response.id,
        first_name: deleted_contact_response.first_name,
        last_name: deleted_contact_response.last_name,
        email: deleted_contact_response.email,
    })
}

pub async fn check_email_exists(email: String) -> Result<bool, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    let result = contact_service.get_contact_by_email(email).await;

    match result {
        Ok(contact) => Ok(true), 
        Err(_) => Ok(false),     
    }
}


pub async fn get_all_contactss() -> Result<Vec<GetContactResponsee>, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);

    let contacts = contact_repository.get_all_contacts().await?;

        println!("Found {} base contacts", contacts.len()); // Add this
    
    let contact_ids = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids).await?;
    
    let list_ids = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(list_ids).await?;
    
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
    overwrite: bool, 
) -> Result<ImportResult, String> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    
    let mut result = ImportResult {
        imported: 0,
        errors: Vec::new(),
    };

    println!("{:?}", contacts);
    println!("{:?}", list_ids);
    println!("Overwrite mode: {}", overwrite);
    
   
    let created_contacts = match contact_repository.upsert_contacts(contacts, overwrite).await {
        Ok(contacts) => {
            result.imported = contacts.len();
            contacts
        },
        Err(e) => {
            return Err(format!("Failed to import contacts: {}", e));
        }
    };
    
    // Associate contacts with lists if any lists were provided
    if !list_ids.is_empty() && !created_contacts.is_empty() {
        let contact_ids: Vec<Uuid> = created_contacts.iter().map(|c| c.id).collect();
        
        for list_id in &list_ids {
            match list_service::add_contacts_to_list(*list_id, contact_ids.clone()).await {
                Ok(_) => { println!("Succesfully added to list") },
                Err(e) => {
                    result.errors.push(format!("Warning: Imported contacts but failed to associate with list {}: {}", list_id, e.to_string()));
                }
            }
        }
    }
    
    Ok(result)
}