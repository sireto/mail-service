// use std::{
//     io::Write,
//     fmt
// };

use chrono::{ DateTime, Utc };
// use serde_json::Value;
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

enum MailStatus {
    Draft,
    Pending,
    Sent,
    Failed,
}

// impl ToString for MailStatus {
//     fn to_string(&self) -> String {
//         match self {
//             MailStatus::Draft => "draft".to_string(),
//             MailStatus::Pending => "pending".to_string(),
//             MailStatus::Sent => "sent".to_string(),
//             MailStatus::Failed => "failed".to_string(),
//         }
//     }
// }

// impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for MailStatus {
//     fn to_sql<'b>(
//         &'b self,
//         out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
//     ) -> diesel::serialize::Result {
//         let status = match self {
//             MailStatus::Draft => "draft",
//             MailStatus::Pending => "pending",
//             MailStatus::Sent => "sent",
//             MailStatus::Failed => "failed",
//         };
        
//         // Use write_all from std::io::Write trait
//         out.write_all(status.as_bytes())
//             .map(|_| diesel::serialize::IsNull::No)
//             .map_err(Into::into)
//     }
// }


// impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for MailStatus {
//     fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
//         // Convert bytes to string first
//         let status_str = std::str::from_utf8(bytes.as_bytes())?;
//         match status_str {
//             "draft" => Ok(MailStatus::Draft),
//             "pending" => Ok(MailStatus::Pending),
//             "sent" => Ok(MailStatus::Sent),
//             "failed" => Ok(MailStatus::Failed),
//             _ => Err(format!("Invalid mail status: {}", status_str).into()),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mail {
    pub id: Uuid,
    pub mail_message: String,
    pub contact_id: Uuid,
    pub template_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,
    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GetMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    pub mail_message: String,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub template_id: Option<Uuid>,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub sent_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::mails)]
pub struct NewMail {
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateMailResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,
    
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
    pub id: Uuid,
    
    pub status: Option<String>,
}
