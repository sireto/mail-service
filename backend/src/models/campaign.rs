use chrono::{ DateTime, NaiveDateTime, Utc };
use serde_json::Value;
use serde::{ Serialize, Deserialize };
use utoipa::{openapi::schema, ToSchema};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::campaigns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Campaign {
    pub id: Uuid,
    pub campaign_name: String,
    pub template_id: Uuid,
    pub namespace_id: Uuid,
    pub status: String,
    pub campaign_senders: Option<Uuid>,
    pub scheduled_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::campaigns)]
pub struct CreateCampaignRequest {
    #[schema(value_type = String, example = "Spring Sale")]
    pub campaign_name: String,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "b2b3b4b5-c2c3-d4d5-e6e7f8f9g0h1")]
    pub namespace_id: Option<Uuid>,

    #[schema(value_type = String, example = "draft")]
    pub status: String,

    #[schema(value_type = String, example = "c3c4c5d6-d7e8f9g0-h1i2j3k4l5m6")]
    pub campaign_senders: Option<Uuid>,

    #[schema(value_type = String, example = "2025-02-10T12:00:00")]
    pub scheduled_at: Option<NaiveDateTime>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetCampaignResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub campaign_name: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub status: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_senders: Option<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub scheduled_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateCampaignResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub campaign_name: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub status: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_senders: Option<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub scheduled_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateCampaignRequest {
    pub campaign_name: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,
    pub status: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_senders: Option<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub scheduled_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateCampaignResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub campaign_name: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Uuid,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub namespace_id: Uuid,
    pub status: String,
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_senders: Option<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub scheduled_at: Option<DateTime<Utc>>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Queryable)]
pub struct DeleteCampaignResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub campaign_name: String,
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct CampaignSendResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: String,
    pub total_recipients: usize,
    pub status: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct CampaignSendRequest {
    #[schema(value_type = String, example = "0a48a82f-ec04-4c19-904c-48dcebc80e49")]
    pub list_id: String,
}