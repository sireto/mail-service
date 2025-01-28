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

pub async fn get_all_lists () -> Result<Vec<List>, diesel::result::Error> {
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
        .load::<List>(&mut conn)
}

pub async fn get_list_by_id(list_id: Uuid) -> Result<List, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    lists
        .filter(id.eq(list_id))
        .first(&mut conn)
}

pub async fn update_list (
    list_id: Uuid, 
    payload: UpdateListRequest
) -> Result<List, diesel::result::Error> {

    let mut conn = get_connection_pool().await;

    diesel::update(lists.find(list_id))
        .set((
            name.eq(payload.name),
            description.eq(payload.description)
        ))
        .get_result(&mut conn)
}

pub async fn delete_list(list_id: Uuid) -> Result<List, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    diesel::delete(lists.filter(id.eq(list_id)))
        .get_result(&mut conn)
}