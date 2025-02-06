use crate::{establish_connection, get_connection_pool};
use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use crate::schema::servers::dsl::*;
use diesel::prelude::*;
use crate::servers::servers_model::{
    Server,
    ServerRequest,
};
use uuid::Uuid;
use mockall::{automock, predicate::*};
use async_trait::async_trait;


#[automock]
#[async_trait]
pub trait ServerRepo {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, diesel::result::Error>;
    async fn get_all_servers(&self) -> Result<Vec<Server>, diesel::result::Error>;
    async fn update_server(&self, server_id: Uuid, payload: ServerRequest) -> Result<Server, diesel::result::Error>;
    async fn delete_server(&self, server_id: Uuid) -> Result<Server, diesel::result::Error>;
    async fn get_server_by_id(&self, server_id: Uuid) -> Result<Server, diesel::result::Error>;
}

pub struct ServerRepoImpl;

#[async_trait]
impl ServerRepo for ServerRepoImpl {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(servers)
            .values(&payload)
            .returning(Server::as_returning())
            .get_result::<Server>(&mut conn)
    }

    async fn get_all_servers(&self) -> Result<Vec<Server>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        servers
            .select((
                id,
                host,
                smtp_username,
                smtp_password,
                namespace_id,
                tls_type,
                port,
                created_at,
                updated_at,
            ))
            .load::<Server>(&mut conn)
    }

    async fn update_server(
        &self,
        server_id: Uuid,
        payload: ServerRequest
    ) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        diesel::update(servers.filter(id.eq(server_id)))
            .set((
                host.eq(&payload.host),
                smtp_username.eq(&payload.smtp_username),
                smtp_password.eq(&payload.smtp_password),
                namespace_id.eq(&payload.namespace_id),
                tls_type.eq(&payload.tls_type),
                port.eq(&payload.port),
            ))
            .get_result(&mut conn)
    }

    async fn delete_server(&self, server_id: Uuid) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::delete(servers.filter(id.eq(server_id)))
            .get_result(&mut conn)
    }

    async fn get_server_by_id(&self, server_id: Uuid) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        servers
            .filter(id.eq(server_id))
            .first(&mut conn)
    }
}