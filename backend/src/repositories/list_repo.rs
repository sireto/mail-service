use crate::models::list::{CreateListRequest, List, UpdateListRequest};
use crate::schema::lists::dsl::*;
use crate::{app_state::DbPooledConnection, GLOBAL_APP_STATE};
use diesel::prelude::*;
use uuid::Uuid;

use async_trait::async_trait;
#[cfg(feature = "mocks")]
use mockall::automock;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait ListRepository {
    async fn create_list(&self, payload: CreateListRequest) -> Result<List, diesel::result::Error>;
    async fn get_all_lists(&self, namespace_uuid: Uuid) -> Result<Vec<List>, diesel::result::Error>;
    async fn get_list_by_id(&self, namespace_uuid: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error>;
    async fn update_list(
        &self,
        namespace_uuid: Uuid,
        list_id: Uuid,
        payload: UpdateListRequest,
    ) -> Result<List, diesel::result::Error>;
    async fn delete_list(&self, namespace_uuid: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error>;
}

pub struct ListRepositoryImpl;

#[async_trait]
impl ListRepository for ListRepositoryImpl {
    async fn create_list(&self, payload: CreateListRequest) -> Result<List, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(lists)
            .values(&payload)
            .returning(List::as_returning())
            .get_result::<List>(&mut conn)
    }

    async fn get_all_lists(&self, namespace_uuid: Uuid) -> Result<Vec<List>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        lists
            .select((id, namespace_id, name, description, created_at, updated_at))
            .filter(namespace_id.eq(namespace_uuid))
            .order(updated_at.desc())
            .load::<List>(&mut conn)
    }

    async fn get_list_by_id(&self, namespace_uuid: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        // Ensure you're querying with both Uuids
        lists
        .filter(namespace_id.eq(namespace_uuid))  // Make sure this is referencing the correct variable
        .filter(id.eq(list_id))  // Filter by list_id as well
        .first(&mut conn)
    }

    async fn update_list(
        &self,
        namespace_uuid: Uuid,
        list_id: Uuid,
        payload: UpdateListRequest,
    ) -> Result<List, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(lists)
            .filter(id.eq(list_id))
            .filter(namespace_id.eq(namespace_uuid))
            .set((
                name.eq(payload.name),
                description.eq(payload.description),
                updated_at.eq(diesel::dsl::now),
            ))
            .get_result(&mut conn)
    }

    async fn delete_list(&self, namespace_uuid: Uuid, list_id: Uuid) -> Result<List, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(lists.filter(namespace_id.eq(namespace_uuid)))
            .filter(id.eq(list_id))
            .get_result(&mut conn)
    }
}
