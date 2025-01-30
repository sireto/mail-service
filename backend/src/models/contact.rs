use chrono::{ DateTime, NaiveDateTime, Utc };
use serde_json::Value;
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub attribute: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::contacts)]
pub struct CreateContactRequest {
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