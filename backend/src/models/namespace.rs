use chrono::{ DateTime, NaiveDateTime, Utc };
use serde_json::Value;
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::namespaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Namespace {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = crate::schema::namespaces)]
pub struct CreateNamespaceRequest {
    #[schema(value_type = String, example = "com.yourcompany.mailservice")]
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateNamespaceResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub name: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateNamespaceRequest {
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateNamespaceResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub name: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>
}