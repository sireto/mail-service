use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use crate::schema::campaign_senders::dsl::*;
use diesel::prelude::*;
use crate::models::campaign_sender::{
    CampaignSender,
    CreateCampaignSenderRequest,
    UpdateCampaignSenderRequest
};
use uuid::Uuid;
use mockall::{automock, predicate::*};
use async_trait::async_trait;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[automock]
#[async_trait]
pub trait CampaignSenderRepository {
    async fn create_campaign_sender(&self, payload: CreateCampaignSenderRequest) -> Result<CampaignSender, diesel::result::Error>;
    async fn get_all_campaign_senders(&self) -> Result<Vec<CampaignSender>, diesel::result::Error>;
    async fn update_campaign_sender(&self, sender_id: Uuid, payload: UpdateCampaignSenderRequest) -> Result<CampaignSender, diesel::result::Error>;
    async fn delete_campaign_sender(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error>;
    async fn get_campaign_sender_by_id(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error>;
}

pub struct CampaignSenderRepositoryImpl;

#[async_trait]
impl CampaignSenderRepository for CampaignSenderRepositoryImpl {
    async fn create_campaign_sender(&self, payload: CreateCampaignSenderRequest) -> Result<CampaignSender, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(campaign_senders)
            .values(&payload)
            .returning(CampaignSender::as_returning())
            .get_result::<CampaignSender>(&mut conn)
    }

    async fn get_all_campaign_senders(&self) -> Result<Vec<CampaignSender>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        campaign_senders
            .select((
                id,
                server_id,
                from_name,
                from_email,
                created_at,
                updated_at,
            ))
            .load::<CampaignSender>(&mut conn)
    }

    async fn update_campaign_sender(
        &self,
        sender_id: Uuid,
        payload: UpdateCampaignSenderRequest
    ) -> Result<CampaignSender, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        diesel::update(campaign_senders.find(sender_id))
            .set((
                from_name.eq(&payload.from_name),
                from_email.eq(&payload.from_email),
            ))
            .get_result(&mut conn)
    }

    async fn delete_campaign_sender(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::delete(campaign_senders.filter(id.eq(sender_id)))
            .get_result(&mut conn)
    }

    async fn get_campaign_sender_by_id(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        campaign_senders
            .filter(id.eq(sender_id))
            .first(&mut conn)
    }
}