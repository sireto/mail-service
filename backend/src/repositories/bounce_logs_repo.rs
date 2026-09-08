use crate::models::bounce_logs::{BounceLog, CreateBounceLogRequest};
use crate::schema::bounce_logs::dsl::*;
use crate::{app_state::DbPooledConnection, GLOBAL_APP_STATE};
use async_trait::async_trait;
use diesel::prelude::*;
#[cfg(feature = "mocks")]
use mockall::automock;
use uuid::Uuid;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait BounceLogsRepository {
    async fn add_bounce(&self, payload: CreateBounceLogRequest) -> Result<BounceLog, diesel::result::Error>;
    async fn get_all_bounces(&self) -> Result<Vec<BounceLog>, diesel::result::Error>;
    async fn delete_bounce(&self, bounce_id: Uuid) -> Result<BounceLog, diesel::result::Error>;
    async fn get_bounces_of_contact_id(&self, bounce_contact_id: Uuid)
        -> Result<Vec<BounceLog>, diesel::result::Error>;
    async fn get_bounce_by_mail_id(&self, bounce_mail_id: String) -> Result<BounceLog, diesel::result::Error>;
}

pub struct BounceLogsRepositoryImpl;

#[async_trait]
impl BounceLogsRepository for BounceLogsRepositoryImpl {
    async fn add_bounce(&self, payload: CreateBounceLogRequest) -> Result<BounceLog, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(bounce_logs)
            .values(&payload)
            .returning(BounceLog::as_returning())
            .get_result::<BounceLog>(&mut conn)
    }

    async fn get_all_bounces(&self) -> Result<Vec<BounceLog>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        bounce_logs
            .select((id, contact_id, mail_id, campaign_id, at, kind, reason))
            .load::<BounceLog>(&mut conn)
    }

    /// Get all bounces of a contact...
    async fn get_bounces_of_contact_id(
        &self,
        bounce_contact_id: Uuid,
    ) -> Result<Vec<BounceLog>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        bounce_logs
            .select((id, contact_id, mail_id, campaign_id, at, kind, reason))
            .filter(contact_id.eq(bounce_contact_id))
            .load::<BounceLog>(&mut conn)
    }

    async fn get_bounce_by_mail_id(&self, bounce_mail_id: String) -> Result<BounceLog, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        bounce_logs
            .filter(mail_id.eq(bounce_mail_id))
            .get_result::<BounceLog>(&mut conn)
    }

    async fn delete_bounce(&self, bounce_id: Uuid) -> Result<BounceLog, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(bounce_logs.filter(id.eq(bounce_id)))
            .returning(BounceLog::as_returning())
            .get_result::<BounceLog>(&mut conn)
    }
}
