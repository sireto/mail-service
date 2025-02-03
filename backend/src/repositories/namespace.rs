use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::namespaces::dsl::*;
use diesel::prelude::*;
use crate::models::namespace::{
    Namespace,
    CreateNamespaceRequest,
    UpdateNamespaceRequest
};
use uuid::Uuid;
use mockall::{ automock, predicate::* };
use async_trait::async_trait;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[automock]
#[async_trait]
pub trait NamespaceRepository {
    async fn create_namespace(&self, payload: CreateNamespaceRequest) -> Result<Namespace, diesel::result::Error>;
    async fn get_all_namespaces(&self) -> Result<Vec<Namespace>, diesel::result::Error>;
    async fn get_namespace_by_id(&self, namespace_id: Uuid) -> Result<Namespace, diesel::result::Error>;
    async fn update_namespace(&self, namespace_id: Uuid, payload: UpdateNamespaceRequest) -> Result<Namespace, diesel::result::Error>;
    async fn delete_namespace(&self, namespace_id: Uuid) -> Result<Namespace, diesel::result::Error>;
}

pub struct NamespaceRepositoryImpl;

#[async_trait]
impl NamespaceRepository for NamespaceRepositoryImpl {
    async fn create_namespace(&self, payload: CreateNamespaceRequest) -> Result<Namespace, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(namespaces)
            .values(&payload)
            .returning(Namespace::as_returning())
            .get_result::<Namespace>(&mut conn)
    }

    async fn get_all_namespaces(&self) -> Result<Vec<Namespace>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        namespaces.select((
            id,
            name,
            created_at,
            updated_at,
            ))
            .load::<Namespace>(&mut conn)
    }

    async fn get_namespace_by_id(&self, namespace_id: Uuid) -> Result<Namespace, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        namespaces
            .find(namespace_id)
            .first(&mut conn)
    }

    async fn update_namespace (&self, namespace_id: Uuid, payload: UpdateNamespaceRequest) -> Result<Namespace, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::update(namespaces.find(namespace_id))
            .set((
                name.eq(&payload.name),
            ))
            .get_result(&mut conn)
    }

    async fn delete_namespace (&self, namespace_id: Uuid) -> Result<Namespace, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::delete(namespaces
            .filter(id.eq(namespace_id)))
            .get_result(&mut conn)
    }
}