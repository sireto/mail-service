use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    pub id: Uuid,
    pub namespace_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub attribute: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::contacts)]
pub struct CreateContactRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,

    #[schema(value_type = String, example = "John")]
    pub first_name: String,

    #[schema(value_type = String, example = "Doe")]
    pub last_name: String,

    #[schema(value_type = String, example = "someone@example.com")]
    pub email: String,

    #[schema(value_type = String, example = "{\"address\": \"Shinjuku\", \"city\": \"Tokyo\"}")]
    pub attribute: Option<Value>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateContactResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub first_name: String,
    pub last_name: String,

    #[schema(value_type = String, example = "someone@example.com")]
    pub email: String,

    #[schema(value_type = String, example = "{\"address\": \"Shinjuku\", \"city\": \"Tokyo\"}")]
    pub attribute: Option<Value>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetContactResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,

    #[schema(value_type = String, example = "{\"address\": \"Shinjuku\", \"city\": \"Tokyo\"}")]
    pub attribute: Option<Value>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub attribute: Option<Value>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateContactResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub attribute: Option<Value>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
// #[diesel(table_name = templates)]
pub struct DeleteContactResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct EmailQuery {
    pub email: String,
    /// Required: an email address is only unique within a namespace.
    #[param(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ContactList {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_id: Uuid,
    #[schema(example = "List A")]
    pub list_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetContactResponsee {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,

    #[schema(value_type = String, example = "{\"address\": \"Shinjuku\", \"city\": \"Tokyo\"}")]
    pub attribute: Option<Value>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,

    #[schema(value_type = Vec<ContactList>)]
    pub lists: Vec<ContactList>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImportOptions {
    pub mode: Option<String>,
    pub status: Option<String>,
    pub overwrite: Option<bool>,
    pub delimiter: Option<String>,
    pub lists: Option<String>, // JSON string of list ids
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImportResponse {
    pub success: bool,
    pub imported: usize,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ImportResult {
    pub imported: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ContactQuery {
    /// Required: contacts are scoped to a namespace.
    #[param(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    #[param(value_type = Option<String>, example = "b1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_id: Option<Uuid>,
    pub search: Option<String>,
}
