use std::{collections::HashMap, sync::Arc};

use crate::{models::{contact::{Contact, ContactList, GetContactResponsee}, list::{CreateListRequest, CreateListResponse, DeleteListResponse, ListResponse, UpdateListRequest, UpdatedListResponse}, list_contacts::NewContactInList}, repositories::{contact::ContactRepositoryImpl, list_repo::{ListRepository, ListRepositoryImpl}}};

use axum::http::StatusCode;
use uuid::Uuid;
use crate::repositories::list_contact_repo::{ListContactRepository, ListContactRepositoryImpl};
use crate::repositories::contact::ContactRepository;


// use anyhow::{anyhow, Result};

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
    ) -> Result<CreateListResponse, (StatusCode, String)> {
        let created_list = self.repository.create_list(payload).await;

        match created_list {
            Ok(list) => Ok(CreateListResponse {
                id: list.id.to_string(),
                name: list.name,
                created_at: list.created_at
            }),
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
        }
    }

    pub async fn get_all_lists(
        &self,
        namespace_id: Uuid,
    ) -> Result<Vec<ListResponse>, (StatusCode, String)> {
        self.repository
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
            })
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }

    pub async fn get_list_by_id(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
    ) -> Result<ListResponse, (StatusCode, String)> {
        self.repository
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
            .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
    }

    pub async fn update_list(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
        payload: UpdateListRequest,
    ) -> Result<UpdatedListResponse, (StatusCode, String)> {
        self.repository
            .update_list(namespace_id, list_id, payload)
            .await
            .map(|list| UpdatedListResponse {
                id: list.id,
                name: list.name,
                updated_at: list.updated_at,
            })
            .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
    }

    pub async fn delete_list(
        &self,
        namespace_id: Uuid,
        list_id: Uuid,
    ) -> Result<DeleteListResponse, (StatusCode, String)> {
        self.repository
            .delete_list(namespace_id, list_id)
            .await
            .map(|list| DeleteListResponse {
                id: list.id,
                name: list.name,
            })
            .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
    }
}

pub async fn create_list(payload: CreateListRequest) -> Result<CreateListResponse, (StatusCode, String)>{
    let new_list = CreateListRequest{
        name: payload.name, 
        namespace_id: payload.namespace_id, 
        description: payload.description, 
    };

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let created_list = list_service.create_list(new_list).await;

    let response_list = match created_list {
        Ok(list) => CreateListResponse {
            id: list.id.to_string(), 
            name: list.name, 
            created_at: list.created_at
        }, 
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("{:?}", err)))
    };
    Ok(response_list)
}

pub async fn get_all_list (namespace_id: Uuid) -> Result<Vec<ListResponse>, (StatusCode, String)>{

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let all_lists = list_service.get_all_lists(namespace_id).await;

    let response_lists = match all_lists {
        Ok(lists) => lists.into_iter().map(|list| ListResponse {
            id: list.id, 
            name: list.name, 
            namespace_id: list.namespace_id, 
            description: list.description, 
            created_at: list.created_at, 
            updated_at: list.updated_at
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("{:?}", err)))
    }; 
    Ok(response_lists)
}

pub async fn get_list_by_id(namespace_id: Uuid, list_id: Uuid) -> Result<ListResponse, (StatusCode, String)> {

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let list = list_service.get_list_by_id(namespace_id, list_id).await;

    match list {
        Ok(list) => {
            let response_list = ListResponse {
                id: list.id,
                name: list.name,
                namespace_id: list.namespace_id,
                description: list.description,
                created_at: list.created_at,
                updated_at: list.updated_at,
            };
            Ok(response_list)
        }
        Err(err) => {
            Err((StatusCode::NOT_FOUND, format!("{:?}", err)))
        }
    }
}


pub async fn update_list(
    namespace_id: String, // Assuming you get the namespace ID as String too
    list_id: String,
    payload: UpdateListRequest,
) -> Result<UpdatedListResponse, (StatusCode, String)> {
    // Convert both 'namespace_id' and 'list_id' (String) to 'Uuid'
    let uuid_namespace_id = Uuid::parse_str(&namespace_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace UUID format".to_string()))?;
    let uuid_list_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid list UUID format".to_string()))?;

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    // Call the repository function to update the list
    let updated_list = list_service.update_list(uuid_namespace_id, uuid_list_id, payload).await;

    // Handle the result from the repository and return a response
    let response_list = match updated_list {
        Ok(list) => UpdatedListResponse {
            id: list.id,
            name: list.name,
            updated_at: list.updated_at,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("{:?}", err))),
    };

    Ok(response_list)
}

pub async fn delete_list (
    namespace_id: String,
    list_id: String,
) -> Result<DeleteListResponse, (StatusCode, String)> {

    let uuid_id = Uuid::parse_str(&list_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let uuid_namespace_id = Uuid::parse_str(&namespace_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let list_repository = Arc::new(ListRepositoryImpl);
    let list_service = ListService::new(list_repository);

    let deleted_list = list_service.delete_list( uuid_namespace_id, uuid_id).await;

    let response_list = match deleted_list {
        Ok(list) => DeleteListResponse {
            id: list.id,
            name: list.name,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("{:?}", err)))
    };

    Ok(response_list)

}

pub async fn add_contacts_to_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<Vec<NewContactInList>, (StatusCode, String)> {
    
let list_contact_repository = Arc::new(ListContactRepositoryImpl);
let contact_service = ListContactService::new(list_contact_repository);

    contact_service.add_contacts_to_list(list_id, contact_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn delete_contacts_from_list(
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<usize, (StatusCode, String)> {
    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let contact_service = ListContactService::new(list_contact_repository);

    contact_service.delete_contacts_from_list(list_id, contact_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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