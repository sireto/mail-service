use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;
use diesel_derive_enum;

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

// Model struct (equivalent to table schema)
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: Uuid,
    pub host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub namespace_id: Uuid,
    pub tls_type: TlsTypeEnum,
    pub port: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// Create DTOs
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::servers)]
pub struct ServerRequest {
    #[schema(example = "smtp.example.com")]
    pub host: String,
    
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
}

// Get DTO
#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct ServerResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub tls_type: TlsTypeEnum,
    pub port: i16,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
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