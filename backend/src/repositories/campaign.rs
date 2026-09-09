use crate::models::campaign::{Campaign, CreateCampaignRequest, UpdateCampaignRequest};
use crate::schema::campaigns::dsl::*;
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
pub trait CampaignRepository {
    async fn create_campaign(&self, payload: CreateCampaignRequest) -> Result<Campaign, diesel::result::Error>;
    /// Returns one page of campaigns plus the total count.
    async fn get_all_campaigns(&self, limit: i64, offset: i64) -> Result<(Vec<Campaign>, i64), diesel::result::Error>;
    async fn update_campaign(
        &self,
        campaign_id: Uuid,
        payload: UpdateCampaignRequest,
    ) -> Result<Campaign, diesel::result::Error>;
    async fn delete_campaign(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error>;
    async fn get_campaign_by_id(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error>;
}

pub struct CampaginRepositoryImpl;

#[async_trait]
impl CampaignRepository for CampaginRepositoryImpl {
    async fn create_campaign(&self, payload: CreateCampaignRequest) -> Result<Campaign, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(campaigns)
            .values(&payload)
            .returning(Campaign::as_returning())
            .get_result::<Campaign>(&mut conn)
    }

    async fn get_all_campaigns(&self, limit: i64, offset: i64) -> Result<(Vec<Campaign>, i64), diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        let total: i64 = campaigns.count().get_result(&mut conn)?;

        campaigns
            .select((
                id,
                campaign_name,
                template_id,
                namespace_id,
                status,
                campaign_senders,
                scheduled_at,
                created_at,
                updated_at,
            ))
            // `id` breaks ties so offset paging is stable across pages.
            .order((updated_at.desc(), id.desc()))
            .limit(limit)
            .offset(offset)
            .load::<Campaign>(&mut conn)
            .map(|items| (items, total))
    }
    async fn update_campaign(
        &self,
        campaign_id: Uuid,
        payload: UpdateCampaignRequest,
    ) -> Result<Campaign, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(campaigns.find(campaign_id))
            .set((
                campaign_name.eq(&payload.campaign_name),
                template_id.eq(&payload.template_id),
                status.eq(&payload.status),
                campaign_senders.eq(&payload.campaign_senders),
                scheduled_at.eq(&payload.scheduled_at),
                updated_at.eq(diesel::dsl::now),
            ))
            .get_result(&mut conn)
    }
    async fn delete_campaign(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(campaigns.filter(id.eq(campaign_id))).get_result(&mut conn)
    }

    async fn get_campaign_by_id(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        campaigns.filter(id.eq(campaign_id)).first(&mut conn)
    }
}
