use chrono::{ DateTime, Utc };
use serde_json::Value;

use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;


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

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub namespace_id: String,
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
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub template_data: Option<Value>,
    pub content_plaintext: Option<String>,
    pub content_html: Option<String>,
}


// the following might change...
#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateTemplateResponse {
    pub id: String,
    pub name: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct DeleteTemplateResponse {
    pub id: String,
    pub name: String,
}