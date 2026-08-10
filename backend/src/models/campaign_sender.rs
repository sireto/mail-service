use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::campaign_senders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CampaignSender {
    pub id: Uuid,
    pub server_id: Uuid,
    pub from_name: String,
    pub from_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::campaign_senders)]
pub struct CreateCampaignSenderRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,

    #[schema(value_type = String, example = "Marketing Team")]
    pub from_name: String,

    #[schema(value_type = String, example = "newsletter@example.com")]
    pub from_email: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct CampaignSenderRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: String,

    #[schema(value_type = String, example = "Marketing Team")]
    pub from_name: String,

    #[schema(value_type = String, example = "newsletter@example.com")]
    pub from_email: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateCampaignSenderResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,
    pub from_name: String,
    pub from_email: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetCampaignSenderResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,
    pub from_name: String,
    pub from_email: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateCampaignSenderRequest {
    pub from_name: String,
    pub from_email: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateCampaignSenderResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,
    pub from_name: String,
    pub from_email: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct DeleteCampaignSenderResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub server_id: Uuid,
    pub from_name: String,
    pub from_email: String,
}

#[derive(Serialize, Debug, Default, Deserialize, ToSchema, Queryable)]
pub struct ValidateEmailIdentityRequest {
    pub email: String,
    #[serde(rename = "serverId")]
    pub server_id: String, // Matches the existing frontend request field name.
}

#[derive(Serialize, Debug, Default, Deserialize, ToSchema, Queryable)]
pub struct ValidateEmailIdentityResponse {
    pub is_valid: bool,
    pub message: Option<String>,
}

#[derive(Serialize, Debug, Default, Deserialize, ToSchema, Queryable)]
pub struct SendTestEmailRequest {
    pub from_email: String,
    pub to_email: String,
    pub from_name: String,
    pub server_id: String,
    pub subject: Option<String>,
}

#[derive(Serialize, Debug, Default, Deserialize, ToSchema, Queryable)]
pub struct SendTestEmailResponse {
    pub success: bool,
    pub message: String,
}
