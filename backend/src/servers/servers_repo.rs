use crate::error::AppError;
use crate::get_connection_pool;
use crate::models::mail::MailWithDetails;
use crate::schema::bounce_logs::dsl as bounce_logs_dsl;
use crate::schema::contacts::dsl as contacts_dsl;
use crate::schema::servers::dsl::*;
use crate::servers::servers_model::{Server, ServerRequest};
use crate::utils::crypto;
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Text};
#[cfg(feature = "mocks")]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(feature = "mocks", automock)]
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

/// Encrypt the credentials on a payload before it is written.
///
/// This is done at the repository boundary so nothing above it has to remember: every
/// caller works with plaintext, and the only values that reach a column are ciphertext.
fn encrypt_payload(mut payload: ServerRequest) -> Result<ServerRequest, diesel::result::Error> {
    payload.smtp_password = crypto::encrypt(&payload.smtp_password).map_err(to_diesel_error)?;
    payload.aws_credentials = crypto::encrypt_aws_credentials(payload.aws_credentials).map_err(to_diesel_error)?;
    Ok(payload)
}

/// Decrypt the credentials on a row that has just been read.
fn decrypt_server(mut server: Server) -> Result<Server, diesel::result::Error> {
    server.smtp_password = crypto::decrypt(&server.smtp_password).map_err(to_diesel_error)?;
    server.aws_credentials = crypto::decrypt_aws_credentials(server.aws_credentials).map_err(to_diesel_error)?;
    Ok(server)
}

/// The repository trait is defined in terms of `diesel::result::Error`, so a crypto failure
/// has to be expressed in those terms rather than widening every signature.
fn to_diesel_error(err: AppError) -> diesel::result::Error {
    diesel::result::Error::SerializationError(Box::new(std::io::Error::other(err.to_string())))
}

#[async_trait]
impl ServerRepo for ServerRepoImpl {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        let payload = encrypt_payload(payload)?;

        diesel::insert_into(servers)
            .values(&payload)
            .returning(Server::as_returning())
            .get_result::<Server>(&mut conn)
            .and_then(decrypt_server)
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
            .load::<Server>(&mut conn)?
            .into_iter()
            .map(decrypt_server)
            .collect()
    }

    async fn update_server(&self, server_id: Uuid, payload: ServerRequest) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        let payload = encrypt_payload(payload)?;

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
            .and_then(decrypt_server)
    }

    async fn delete_server(&self, server_id: Uuid) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(servers.filter(id.eq(server_id)))
            .get_result(&mut conn)
            .and_then(decrypt_server)
    }

    async fn get_server_by_id(&self, server_id: Uuid) -> Result<Server, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        servers
            .filter(id.eq(server_id))
            .first(&mut conn)
            .and_then(decrypt_server)
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
                sql::<Nullable<Text>>("NULL"),
            ))
            .load::<MailWithDetails>(&mut conn)
    }
}
