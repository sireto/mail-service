use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::{pg::Pg, prelude::*};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

enum MailStatus {
    Draft,
    Pending,
    Sent,
    Bounced,
}

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mail {
    pub id: String,
    pub mail_message: String,
    pub contact_id: Uuid,
    pub template_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,
    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Queryable, QueryableByName, ToSchema )]
#[diesel(check_for_backend(Pg))] // Ensure this struct is valid for PostgreSQL...
#[diesel(table_name = crate::schema::mails)]
pub struct MailWithDetails {
    pub id: String,

    #[diesel(sql_type = diesel::sql_types::Text)]
    pub mail_message: String,

    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Uuid>)]
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Uuid>)]
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,

    #[diesel(sql_type = diesel::sql_types::Text)]
    pub status: String,

    #[diesel(sql_type = diesel::sql_types::Text)]
    pub email: String,  // This comes from contacts.email

    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub reason: Option<String>,  // This comes from bounce_logs.reason
}


#[derive(Debug, Default, Serialize, Deserialize, ToSchema )]
pub struct GetMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: String,
    pub mail_message: String,
    
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub email: String,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,
    pub status: String,
    pub status_reason: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::mails)]
pub struct NewMail {
    pub id: String,
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

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq )]

pub struct CreateMailRequest {
    pub id: String,
    pub mail_message: String,

    #[schema(value_type = Vec<String>, example = "someone@example.com")]
    pub email: Vec<String>,
    // #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    // pub contact_id: Uuid,

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
    pub id: String,

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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: String,
    
    pub mail_message: String,

    #[schema(value_type = String, example = "c1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "d2a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,
    pub status: Option<String>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: String,
    
    pub status: Option<String>,
}

/**
 * query struct for mail query...
 */
#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct MailQuery {
    #[schema(value_type = String, example = "[a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8, a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8]")]
    pub campaign_ids: Option<Uuid>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub from: Option<DateTime<Utc>>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub to: Option<DateTime<Utc>>,
}