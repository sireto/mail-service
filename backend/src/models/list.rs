use chrono::{ DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use diesel::prelude::*;

use serde::{Serialize, Deserialize};
use utoipa::{openapi::schema, ToSchema};

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::lists)]

pub struct CreateListRequest {
    pub name: String, 
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid, 
    pub description: String, 
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateListResponse {
    pub id: String, 
    pub name: String, 
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[primary_key(id)] 
pub struct List {
    pub id: Uuid, 
    pub namespace_id: Uuid, 
    pub name: String, 
    pub description: Option<String>,
    pub created_at: NaiveDateTime, 
    pub updated_at: NaiveDateTime, 
}


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct ListResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid, 
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid, 
    pub name: String, 
    pub description: Option<String>, 
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: NaiveDateTime, 
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateListRequest {
    pub name: String, 
    pub description: Option<String>, 
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct UpdatedListResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid, 
    pub name: String, 
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
// #[diesel(table_name = templates)]
pub struct DeleteListResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub name: String,
}