use chrono::{ DateTime, NaiveDateTime, Utc };
use serde_json::Value;
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