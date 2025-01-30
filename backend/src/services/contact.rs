use crate::repositories::contact::{self, ContactRepository, ContactRepositoryImpl};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::contact::{
    Contact,
    CreateContactRequest,
    CreateContactResponse, GetContactResponse, UpdateContactRequest, UpdateContactResponse,
    DeleteContactResponse
};

pub struct ContactService {
    repository: Arc<dyn ContactRepository + Send + Sync>
}

impl ContactService {
    pub fn new(repository: Arc<dyn ContactRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_contact(&self, payload: CreateContactRequest) -> Result<Contact, diesel::result::Error> {
        self.repository.create_contact(payload).await
    }

    pub async fn get_all_contacts(&self) -> Result<Vec<Contact>, diesel::result::Error> {
        self.repository.get_all_contacts().await
    }

    pub async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        self.repository.get_contact_by_id(contact_id).await
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


pub async fn create_contact(payload: CreateContactRequest) -> Result<CreateContactResponse, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let response = contact_service.create_contact(payload).await;

    let response = match response {
        Ok(contact) => CreateContactResponse {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

pub async fn get_all_contacts() -> Result<Vec<GetContactResponse>, (StatusCode, String)> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let all_contacts = contact_service.get_all_contacts().await;

    let response = match all_contacts {
        Ok(contacts) => contacts.into_iter().map(|contact| GetContactResponse {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response)
}

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