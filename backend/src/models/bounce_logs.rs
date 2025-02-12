use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };
use utoipa::ToSchema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::bounce_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BounceLog {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub at: DateTime<Utc>,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetBounceLogResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub at: DateTime<Utc>,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::bounce_logs)]

pub struct CreateBounceLogRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub at: DateTime<Utc>,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct CreateBounceLogResponse {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub id: Uuid,

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact: Uuid,    // {email} from the contact id or shall we use the contact_id itself...

    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Option<Uuid>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub at: DateTime<Utc>,
    pub kind: String,
    pub reason: String,
}

// bounce SNS notification models...

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SnsNotification {
    #[serde(rename = "Type")]
    pub notification_type: String,
    #[serde(rename = "Message")]
    pub message: String, // Actual bounce data in JSON string format...
    #[serde(rename = "MessageId")]
    pub message_id: String,
    // #[serde(rename = "SubscribeURL")]
    // pub subscribe_url: String,

}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BounceNotification {
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    pub bounce: BounceDetails,
    pub mail: MailDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BounceDetails {
    #[serde(rename = "bounceType")]
    pub bounce_type: String,
    #[serde(rename = "bounceSubType")]
    pub bounce_sub_type: String,
    #[serde(rename = "bouncedRecipients")]
    pub bounced_recipients: Vec<BouncedRecipient>,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BouncedRecipient {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct MailDetails {
    pub source: String,
    pub destination: Vec<String>,
}
