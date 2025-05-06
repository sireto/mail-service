use crate::models::mail::MailWithDetails;
use crate::{establish_connection, get_connection_pool};
use crate::schema::servers::dsl::*;
use diesel::prelude::*;
use crate::servers::servers_model::{
    Server,
    ServerRequest,
};
use crate::schema::contacts::dsl as contacts_dsl;
use crate::schema::bounce_logs::dsl as bounce_logs_dsl;
use uuid::Uuid;
use diesel::dsl::sql;
use diesel::sql_types::{ Nullable, Text };
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
    async fn get_mails_by_server_id(&self, server_id: Uuid) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
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
                active,
                host,
                smtp_username,
                smtp_password,
                namespace_id,
                tls_type,
                port,
                server_type, 
                aws_credentials,
                created_at,
                updated_at,
                default_from_email,
                rate_limit,
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
                active.eq(&payload.active),
                host.eq(&payload.host),
                smtp_username.eq(&payload.smtp_username),
                smtp_password.eq(&payload.smtp_password),
                namespace_id.eq(&payload.namespace_id),
                tls_type.eq(&payload.tls_type),
                server_type.eq(&payload.server_type), 
                aws_credentials.eq(&payload.aws_credentials),
                port.eq(&payload.port),
                default_from_email.eq(&payload.default_from_email),
                rate_limit.eq(&payload.rate_limit),
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

    async fn get_mails_by_server_id(&self, server_id_arg: Uuid) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        use crate::schema::mails::dsl::*;

        let mut conn = get_connection_pool().await;
        
       // get mails only related to that server with the help of the server_id column in the mails table...

        mails
            .filter(server_id.eq(server_id_arg))
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .left_outer_join(bounce_logs_dsl::bounce_logs.on(id.eq(bounce_logs_dsl::mail_id)))
            .select((
                id,
                mail_message,
                template_id,
                campaign_id,
                server_id,
                sent_at,
                status,
                open,
                clicks,
                scheduled_at,
                attempts,
                last_error,
                contacts_dsl::email,
                bounce_logs_dsl::reason.nullable(),
                sql::<Nullable<Text>>("NULL")
            ))
            .load::<MailWithDetails>(&mut conn)
    }
}