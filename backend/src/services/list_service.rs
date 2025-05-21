use std::{collections::HashMap, sync::Arc};

use crate::{error::AppError, models::{contact::{Contact, ContactList, GetContactResponsee}, list::{CreateListRequest, CreateListResponse, DeleteListResponse, ListResponse, UpdateListRequest, UpdatedListResponse}, list_contacts::NewContactInList}, repositories::{contact::ContactRepositoryImpl, list_repo::{ListRepository, ListRepositoryImpl}}};

use axum::http::StatusCode;
use uuid::Uuid;
use crate::repositories::list_contact_repo::{ListContactRepository, ListContactRepositoryImpl};
use crate::repositories::contact::ContactRepository;


pub struct ListContactService {
    repository: Arc<dyn ListContactRepository + Send + Sync>
}
pub struct ListService{
    repository: Arc<dyn ListRepository + Send +Sync>
}



impl ListContactService {
    pub fn new(repository: Arc<dyn ListContactRepository + Send + Sync >) -> Self {
        Self{repository}
    }
    pub async fn add_contacts_to_list(&self, list_id: Uuid, contact_id: Vec<Uuid>) -> Result<Vec<NewContactInList>, diesel::result::Error>{
        self.repository.add_contacts_to_list(list_id, contact_id).await
    }
    pub async fn delete_contacts_from_list(&self, list_id: Uuid, contact_id: Vec<Uuid>) -> Result<usize, diesel::result::Error>{
        self.repository.delete_contacts_from_list(list_id, contact_id).await
    }
    pub async fn get_contacts_from_lists(&self, lists:  Vec<Uuid>) -> Result<Vec<Contact>, diesel::result::Error>{
        self.repository.get_contacts_from_lists(lists).await
    }
}

impl ListService {
    pub fn new(repository: Arc<dyn ListRepository + Send +Sync>) -> Self {
        Self { repository}
    }
    pub async fn create_list(
        &self,
        payload: CreateListRequest,
    ) -> Result<CreateListResponse, AppError> {
        let created_list = self.repository.create_list(payload).await?;

        Ok(CreateListResponse {
            id: created_list.id.to_string(),
            name: created_list.name,
            created_at: created_list.created_at,
        })
    }

    pub async fn get_all_lists(
        &self,
        namespace_id: Uuid,
    ) -> Result<Vec<ListResponse>, AppError> {
        let all_lists = self.repository
            .get_all_lists(namespace_id)
            .await
            .map(|lists| {
                lists
                    .into_iter()
                    .map(|list| ListResponse {
                        id: list.id,
                        name: list.name,
                        namespace_id: list.namespace_id,
                        description: list.description,
                        created_at: list.created_at,
                        updated_at: list.updated_at,
                    })
                    .collect()
            })?;

        Ok(all_lists)
    }

    pub async fn get_list_by_id(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
    ) -> Result<ListResponse, AppError> {
        let list = self.repository
            .get_list_by_id(namespace_id, list_id)
            .await
            .map(|list| ListResponse {
                id: list.id,
                name: list.name,
                namespace_id: list.namespace_id,
                description: list.description,
                created_at: list.created_at,
                updated_at: list.updated_at,
            })
            .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

        Ok(list)
    }

    pub async fn update_list(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
        payload: UpdateListRequest,
    ) -> Result<UpdatedListResponse, AppError> {
        let updated_list = self.repository
            .update_list(namespace_id, list_id, payload)
            .await
            .map(|list| UpdatedListResponse {
                id: list.id,
                name: list.name,
                updated_at: list.updated_at,
            })
            .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

        Ok(updated_list)
    }

    pub async fn delete_list(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
    ) -> Result<DeleteListResponse, AppError> {
        let deleted_list = self.repository
            .delete_list(namespace_id, list_id)
            .await
            .map(|list| DeleteListResponse {
                id: list.id,
                name: list.name,
            })
            .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

        Ok(deleted_list)
    }
}

pub async fn create_list(payload: CreateListRequest) -> Result<CreateListResponse, AppError>{
    let new_list = CreateListRequest{
        name: payload.name, 
        namespace_id: payload.namespace_id, 
        description: payload.description, 
    };

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let created_list = list_service.create_list(new_list).await?;

    Ok(CreateListResponse{
        id: created_list.id.to_string(), 
        name: created_list.name, 
        created_at: created_list.created_at
    })
}

pub async fn get_all_list (namespace_id: Uuid) -> Result<Vec<ListResponse>, AppError>{

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let all_lists = list_service.get_all_lists(namespace_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let lists = all_lists.into_iter().map(|list| ListResponse{
        id: list.id, 
        name: list.name, 
        namespace_id: list.namespace_id, 
        description: list.description, 
        created_at: list.created_at, 
        updated_at: list.updated_at
    }).collect();

    Ok(lists)
}

pub async fn get_list_by_id(namespace_id: Uuid, list_id: Uuid) -> Result<ListResponse, AppError> {

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let list = list_service.get_list_by_id(namespace_id, list_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(ListResponse{
        id: list.id, 
        name: list.name, 
        namespace_id: list.namespace_id, 
        description: list.description, 
        created_at: list.created_at, 
        updated_at: list.updated_at
    })
}


pub async fn update_list(
    namespace_id: Uuid, // Assuming you get the namespace ID as Uuid too
    list_id: Uuid,
    payload: UpdateListRequest,
) -> Result<UpdatedListResponse, AppError> {
    // Convert both 'namespace_id' and 'list_id' (String) to 'Uuid'
    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    // Call the repository function to update the list
    let updated_list = list_service.update_list(namespace_id, list_id, payload).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(UpdatedListResponse {
        id: updated_list.id,
        name: updated_list.name,
        updated_at: updated_list.updated_at,
    })
}

pub async fn delete_list (
    namespace_id: Uuid,
    list_id: Uuid,
) -> Result<DeleteListResponse, AppError> {
    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let deleted_list = list_service.delete_list(namespace_id, list_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(DeleteListResponse {
        id: deleted_list.id,
        name: deleted_list.name,
    })
}

pub async fn add_contacts_to_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<Vec<NewContactInList>, AppError> {
    
let list_contact_repository = Arc::new(ListContactRepositoryImpl);
let contact_service = ListContactService::new(list_contact_repository);
    let new_contact_in_list = contact_service.add_contacts_to_list(list_id, contact_ids)
        .await?;

    Ok(new_contact_in_list)
}

pub async fn delete_contacts_from_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<usize, AppError> {
    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let contact_service = ListContactService::new(list_contact_repository);

    let deleted_contact_from_list = contact_service.delete_contacts_from_list(list_id, contact_ids)
        .await?;

    Ok(deleted_contact_from_list)
}

pub async fn get_contacts_from_lists(
    list_ids: Vec<Uuid>,
) -> Result<Vec<GetContactResponsee>, (StatusCode, String)> {
    
    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let contact_service = ListContactService::new(list_contact_repository);
    let contact_repository = Arc::new(ContactRepositoryImpl);

    // Get all contacts from the specified lists
    let contacts = contact_service.get_contacts_from_lists(list_ids.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if contacts.is_empty() {
        return Ok(Vec::new());
    }
    
    // Get all list_contacts for these contacts
    let contact_ids: Vec<Uuid> = contacts.iter().map(|c| c.id).collect();
    let list_contacts = contact_repository.get_list_contacts(contact_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Get all lists these contacts belong to
    let all_list_ids: Vec<Uuid> = list_contacts.iter().map(|lc| lc.list_id).collect();
    let lists = contact_repository.get_lists_by_ids(all_list_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Create a map of list_id to list_name
    let list_map: HashMap<Uuid, String> = lists.into_iter()
        .map(|list| (list.id, list.name))
        .collect();

    // Construct the response with list names for each contact
    let mut response = Vec::new();
    for contact in contacts {
        let contact_lists = list_contacts.iter()
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
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            attribute: contact.attribute,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            lists: contact_lists,
        });
    }

    Ok(response)
}