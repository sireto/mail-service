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
 

 use diesel::prelude::*;
 use uuid::Uuid;
 
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
     #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
     pub id: Uuid,
 
     pub list: String,
     pub from: String,    
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