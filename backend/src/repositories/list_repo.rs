use crate::models::list_contacts::NewContactInList;
use crate::schema::{lists, templates};
use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use crate::schema::lists::dsl::*;
use diesel::prelude::*;
use crate::models::list::{CreateListRequest, CreateListResponse, List, UpdateListRequest};
use uuid::Uuid;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

pub async fn create_list (
    payload: CreateListRequest
) -> Result<List, diesel::result::Error> {

    let mut conn = get_connection_pool().await;

    diesel::insert_into(lists)
        .values(&payload)
        .returning(List::as_returning())
        .get_result::<List>(&mut conn)
}

pub async fn get_all_lists(namespaceId: Uuid) -> Result<Vec<List>, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    lists
        .select((
            id, 
            namespace_id, 
            name, 
            description, 
            created_at, 
            updated_at
        ))
        .filter(namespace_id.eq(namespaceId))
        .load::<List>(&mut conn)
}

pub async fn get_list_by_id(namespaceId: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    // Ensure you're querying with both Uuids
    lists
        .filter(namespace_id.eq(namespaceId))  // Make sure this is referencing the correct variable
        .filter(id.eq(list_id))  // Filter by list_id as well
        .first(&mut conn)
}

pub async fn update_list (
    namespaceId: Uuid, 
    list_id: Uuid, 
    payload: UpdateListRequest
) -> Result<List, diesel::result::Error> {

    let mut conn = get_connection_pool().await;

    diesel::update(lists)
            .filter(id.eq(list_id))
            .filter(namespace_id.eq(namespaceId))
            .set((
                name.eq(payload.name), 
                description.eq(payload.description)
            ))
            .get_result(&mut conn)
}

pub async fn delete_list(namespaceId: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    diesel::delete(lists.filter(namespace_id.eq(namespaceId)))
            .filter(id.eq(list_id))
        .get_result(&mut conn)
}


