/**
 * The following model was the schema for the template table in the prisma-client-rust which was in the backend/src/schema.rs file...
 */

use chrono::{ DateTime, NaiveDateTime, Utc };
use serde_json::Value;

use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::templates)]
pub struct CreateTemplateRequest {
    pub name: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub template_data: Value,
    pub content_plaintext: Option<String>,
    pub content_html: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]

pub struct CreateTemplateResponse {
    pub id: String,
    pub name: String,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct TemplateResponse {
    pub id: String,
    pub name: String,
    pub namespace_id: String,
    pub template_data: Value,
    pub content_plaintext: Option<String>,
    pub content_html: String,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetTemplateResponse{
    pub id: String, 
    pub name: String, 
    pub namespace_id: String, 
    pub template_data: Value, 
    pub content_plaintext: Option<String>, 
    pub content_html: String, 
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>, 
    #[schema(value_type = String, example="2023-01-10T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

// #[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
// pub struct UpdateTemplateRequest {
//     pub name: Option<String>,
//     pub template_data: Option<Value>,
//     pub content_plaintext: Option<String>,
//     pub content_html: Option<String>,
// }


// the following might change...
// #[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
// pub struct UpdateTemplateResponse {
//     pub id: String,
//     pub name: String,

//     #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
//     pub updated_at: DateTime<Utc>,
// }

// #[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
// pub struct DeleteTemplateResponse {
//     pub id: Uuid,
//     pub name: String,
//     pub updated_at: NaiveDateTime,
// }

/*
    <==== Here the following models are for the diesel ORM... ======>
*/
use diesel::prelude::*;
use crate::schema::templates;
// use diesel::pg::sql_types::Uuid;
use uuid::Uuid;

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = templates)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Template {
//     pub id: Uuid,
//     pub namespace_id: Uuid,
//     pub name: String,
//     pub template_data: Value,
//     pub content_plaintext: Option<String>,
//     pub content_html: String,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[primary_key(id)] 
pub struct Template {
    pub id: Uuid,
    pub namespace_id: Uuid,
    pub name: String,
    pub template_data: Value,
    pub content_plaintext: Option<String>,
    pub content_html: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateTemplateRequest {
    pub name: String,
    pub template_data: Value,
    pub content_html: String,
    pub content_plaintext: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct UpdateTemplateResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub name: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
// #[diesel(table_name = templates)]
pub struct DeleteTemplateResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct SendMailRequest {
    pub receiver: Option<String>,   // this should be a list of emails seperated by commas or the list name for now (later to be changed to the list_id)...
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub from: String, 
    pub subject: String,   
    pub template_data: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct SendMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub name: String,
    pub to: Vec<String>,
    pub from: String,
}