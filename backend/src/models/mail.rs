use chrono::{ DateTime, Utc };
use serde_json::Value;
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mail {
    pub id: Uuid,
    pub mail_message: String,
    pub contact_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,

    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::mails)]
pub struct CreateMailRequest {
    pub mail_message: String,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,

    #[schema(value_type = String, example = "b1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "c1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    pub mail_message: String,

    #[schema(value_type = String, example = "b1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,

    #[schema(value_type = String, example = "c1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "d2a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateMailRequest {
    pub mail_message: String,

    #[schema(value_type = String, example = "c1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "d2a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,
    pub status: Option<String>,
}
