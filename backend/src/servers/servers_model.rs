use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;
use diesel_derive_enum;
use crate::utils::server_utils::secure_server_response;

// Assuming TlsTypeEnum is defined elsewhere with proper diesel and serde derivations
#[derive(Debug, Clone, PartialEq, diesel_derive_enum::DbEnum, Serialize, Deserialize, ToSchema, Default)]
#[ExistingTypePath = "crate::schema::sql_types::TlsType"]
pub enum TlsTypeEnum {
    #[serde(rename = "NONE")]
    #[db_rename = "NONE"]
    #[default]
    NONE,
    #[serde(rename = "STARTTLS")]
    #[db_rename = "STARTTLS"]
    STARTTLS,
    #[serde(rename = "SSL/TLS")]
    #[db_rename = "SSL/TLS"]
    SSLTLS,
}

#[derive(Debug, Clone, Copy, PartialEq, diesel_derive_enum::DbEnum, Serialize, Deserialize, ToSchema, Default)]
#[ExistingTypePath = "crate::schema::sql_types::ServerType"]
pub enum ServerTypeEnum {
    #[serde(rename = "SMTP")]
    #[db_rename = "SMTP"]
    #[default]
    SMTP,
    #[serde(rename = "AWS")]
    #[db_rename = "AWS"]
    AWS,
}

// Model struct (equivalent to table schema)
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: Uuid,
    pub active: bool,
    pub host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub namespace_id: Uuid,
    pub tls_type: TlsTypeEnum,
    pub port: i16,
    pub server_type: ServerTypeEnum,
    pub aws_credentials: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub default_from_email: String,
}
// Create DTOs
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::servers)]
pub struct ServerRequest {
    #[schema(example = "smtp.example.com")]
    pub host: String,

    #[schema(example="true")]
    pub active: bool,
    
    #[schema(example = "user@example.com")]
    pub smtp_username: String,
    
    #[schema(example = "password123")]
    pub smtp_password: String,
    
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    
    #[schema(value_type = TlsTypeEnum, example = "Starttls")]
    pub tls_type: TlsTypeEnum,
    
    #[schema(example = 587)]
    pub port: i16,

    #[schema(value_type = ServerTypeEnum, example = "SMTP")]
    pub server_type: ServerTypeEnum,

    pub aws_credentials: Option<serde_json::Value>,
    
    #[schema(example = "from.email@test.io")]
    pub default_from_email: String,
}

// Get DTO
#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct ServerResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(example="true")]
    pub active: bool,
    pub host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub tls_type: TlsTypeEnum,
    pub port: i16,
    pub server_type: ServerTypeEnum,

    pub aws_credentials: Option<serde_json::Value>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
    #[schema(example = "from.email@test.io")]
    pub default_from_email: String,
}

impl From<Server> for ServerResponse {
    fn from(server: Server) -> Self {
        let safe_aws = server.aws_credentials.map(|creds| secure_server_response(creds));

        Self {
            id: server.id,
            active: server.active,
            host: server.host,
            smtp_username: server.smtp_username,
            smtp_password: "*************".to_string(),
            namespace_id: server.namespace_id,
            tls_type: server.tls_type,
            port: server.port,
            server_type: server.server_type,
            aws_credentials: safe_aws,
            created_at: server.created_at,
            updated_at: server.updated_at,
            default_from_email: server.default_from_email,
        }
    }
}


// Delete DTO
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct DeleteServerResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub host: String,
    pub port: i16,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct SendMailFromServerRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,

    pub receiver: Option<String>,   // this should be a list of emails seperated by commas or the list name for now (later to be changed to the list_id)...
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct SendMailFromServerResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,   // mail id...
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub mail_send_ids: Vec<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,
    pub status: String,
}