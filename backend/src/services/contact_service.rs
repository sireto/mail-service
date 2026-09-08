use super::{list_service, mail_service::MailServiceTrait};
use crate::error::AppError;
use crate::models::contact::{
    Contact, CreateContactRequest, CreateContactResponse, DeleteContactResponse, GetContactResponse, ImportResult,
    UpdateContactRequest, UpdateContactResponse,
};
use crate::models::mail::MailWithDetails;
use crate::models::pagination::{Page, PageQuery};
use crate::repositories::mail_repository::MailRepositoryImpl;
use crate::services::mail_service;
use crate::{
    models::contact::{ContactList, GetContactResponsee},
    repositories::contact::{ContactRepository, ContactRepositoryImpl},
};
use email_address::EmailAddress;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub struct ContactService {
    repository: Arc<dyn ContactRepository + Send + Sync>,
}

impl ContactService {
    pub fn new(repository: Arc<dyn ContactRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_contacts(
        &self,
        payload: Vec<CreateContactRequest>,
    ) -> Result<Vec<Contact>, diesel::result::Error> {
        self.repository.create_contacts(payload).await
    }

    pub async fn get_all_contacts(
        &self,
        namespace: Uuid,
        list_id: Option<Uuid>,
        search: Option<String>,
        page: &PageQuery,
    ) -> Result<Page<Contact>, diesel::result::Error> {
        let (limit, offset) = (page.limit(), page.offset());
        let (items, total) = self
            .repository
            .get_all_contacts(namespace, list_id, search, limit, offset)
            .await?;

        Ok(Page::new(items, total, limit, offset))
    }

    pub async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        self.repository.get_contact_by_id(contact_id).await
    }

    pub async fn get_contact_by_email(&self, namespace: Uuid, email: String) -> Result<Contact, diesel::result::Error> {
        self.repository.get_contact_by_email(namespace, email).await
    }

    pub async fn update_contact(
        &self,
        contact_id: Uuid,
        payload: UpdateContactRequest,
    ) -> Result<Contact, diesel::result::Error> {
        self.repository.update_contact(contact_id, payload).await
    }

    pub async fn delete_contact(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        self.repository.delete_contact(contact_id).await
    }
}

/// Reject malformed addresses before they reach the database.
///
/// email was a plain String with no validation anywhere, so an invalid address was stored
/// happily and only surfaced later as a provider rejection or, on the SMTP path, a panic
/// while parsing the recipient.
pub fn validate_contact_email(email: &str) -> Result<String, AppError> {
    let trimmed = email.trim();

    if trimmed.is_empty() {
        return Err(AppError::BadRequestError(Some("email is required".to_string())));
    }

    if !EmailAddress::is_valid(trimmed) {
        return Err(AppError::BadRequestError(Some(format!(
            "{trimmed:?} is not a valid email address"
        ))));
    }

    Ok(trimmed.to_string())
}

pub async fn create_contacts(
    mut payloads: Vec<CreateContactRequest>, // Corrected syntax
) -> Result<Vec<CreateContactResponse>, AppError> {
    for payload in payloads.iter_mut() {
        payload.email = validate_contact_email(&payload.email)?;
    }

    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let created_contacts = contact_service.create_contacts(payloads).await?;

    let response = created_contacts
        .into_iter()
        .map(|contact| CreateContactResponse {
            id: contact.id,
            namespace_id: contact.namespace_id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
        })
        .collect();

    Ok(response)
}

pub async fn get_all_contacts(
    namespace: Uuid,
    list_id: Option<Uuid>,
    search: Option<String>,
    page: &PageQuery,
) -> Result<Page<GetContactResponsee>, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let (limit, offset) = (page.limit(), page.offset());

    let (contacts, total) = contact_repository
        .get_all_contacts(namespace, list_id, search, limit, offset)
        .await?;

    // These two lookups are bounded by the page: they were the reason this endpoint pulled
    // every list_contacts row and every list into memory, which it only did because
    // `contacts` itself was unbounded.
    let contact_ids = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids).await?;

    let list_ids = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(list_ids).await?;

    let list_map: HashMap<Uuid, String> = lists.into_iter().map(|list| (list.id, list.name)).collect();

    let mut response = Vec::new();
    for contact in contacts {
        let contact_lists = list_contacts
            .iter()
            .filter(|lc| lc.contact_id == contact.id)
            .filter_map(|lc| {
                list_map.get(&lc.list_id).map(|name| ContactList {
                    list_id: lc.list_id,
                    list_name: name.clone(),
                })
            })
            .collect::<Vec<ContactList>>();

        response.push(GetContactResponsee {
            id: contact.id,
            namespace_id: contact.namespace_id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            lists: contact_lists,
        });
    }

    Ok(Page::new(response, total, limit, offset))
}

/// function to get the contact by email...
pub async fn get_contact_by_id(contact_id: Uuid) -> Result<GetContactResponse, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_id(contact_id).await;

    let contact = contact.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let contact_response = GetContactResponse {
        id: contact.id,
        namespace_id: contact.namespace_id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
        created_at: contact.created_at,
        updated_at: contact.updated_at,
    };

    Ok(contact_response)
}

/// Look a contact up by address within a namespace.
///
/// The namespace is not optional: since contacts_namespace_id_email_key replaced the global
/// unique constraint on email, the same address can exist in more than one namespace and an
/// unqualified lookup would return an arbitrary one of them.
pub async fn get_contact_by_email(namespace: Uuid, email: String) -> Result<GetContactResponse, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);
    let contact = contact_service.get_contact_by_email(namespace, email).await;

    let contact = contact.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let contact_response = GetContactResponse {
        id: contact.id,
        namespace_id: contact.namespace_id,
        first_name: contact.first_name,
        last_name: contact.last_name,
        email: contact.email,
        attribute: contact.attribute,
        created_at: contact.created_at,
        updated_at: contact.updated_at,
    };

    Ok(contact_response)
}

pub async fn update_contact(
    contact_id: Uuid,
    mut payload: UpdateContactRequest,
) -> Result<UpdateContactResponse, AppError> {
    payload.email = validate_contact_email(&payload.email)?;

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

pub async fn delete_contact(contact_id: Uuid) -> Result<DeleteContactResponse, AppError> {
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

pub async fn check_email_exists(namespace: Uuid, email: String) -> Result<bool, AppError> {
    let contact_repository = Arc::new(ContactRepositoryImpl);
    let contact_service = ContactService::new(contact_repository);

    // Scoped to the namespace: the same address existing in another namespace is not a
    // duplicate here, and reporting it as one would leak the other tenant's contact list.
    let result = contact_service.get_contact_by_email(namespace, email).await;

    match result {
        Ok(_contact) => Ok(true),
        Err(_) => Ok(false),
    }
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

    // Partition the file into rows we can store and rows we must report back. Previously
    // every row went straight to the database unvalidated, and the entire batch (names and
    // addresses) was printed to stdout.
    let mut valid_contacts = Vec::with_capacity(contacts.len());
    for contact in contacts {
        match validate_contact_email(&contact.email) {
            Ok(email) => valid_contacts.push(CreateContactRequest { email, ..contact }),
            Err(err) => result.errors.push(format!("Skipped {:?}: {err}", contact.email)),
        }
    }

    println!(
        "Importing {} contacts into {} list(s), overwrite={}",
        valid_contacts.len(),
        list_ids.len(),
        overwrite
    );

    if valid_contacts.is_empty() {
        return Ok(result);
    }

    let created_contacts = match contact_repository.upsert_contacts(valid_contacts, overwrite).await {
        Ok(contacts) => {
            result.imported = contacts.len();
            contacts
        }
        Err(e) => {
            return Err(format!("Failed to import contacts: {}", e));
        }
    };

    // Associate contacts with lists if any lists were provided
    if !list_ids.is_empty() && !created_contacts.is_empty() {
        let contact_ids: Vec<Uuid> = created_contacts.iter().map(|c| c.id).collect();

        for list_id in &list_ids {
            match list_service::add_contacts_to_list(*list_id, contact_ids.clone()).await {
                Ok(_) => {
                    println!("Succesfully added to list")
                }
                Err(e) => {
                    result.errors.push(format!(
                        "Warning: Imported contacts but failed to associate with list {}: {}",
                        list_id, e
                    ));
                }
            }
        }
    }

    Ok(result)
}

pub async fn get_mails_for_contact(contact_id: Uuid) -> Result<Vec<MailWithDetails>, AppError> {
    let mail_repo = Arc::new(MailRepositoryImpl);
    let mail_service = mail_service::MailService::new(mail_repo);

    let mails = mail_service.get_mails_by_contact(contact_id).await?;

    Ok(mails)
}
