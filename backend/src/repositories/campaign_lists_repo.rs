use crate::models::campaign::Campaign;
use crate::models::list::ListResponse;
use crate::models::{campaign_lists::NewListInCampaign, list::List};
use crate::schema::campaign_lists;
use crate::schema::lists::dsl::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use async_trait::async_trait;
use mockall::{automock, predicate::*};

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[automock]
#[async_trait]
pub trait CampaignListRepository {
    async fn add_lists_to_campaign(
        &self,
        campaign_id: Uuid,
        list_id: Vec<Uuid>,
    ) -> Result<Vec<NewListInCampaign>, diesel::result::Error>;
    async fn delete_lists_from_campaign(
        &self,
        campaign_id: Uuid,
        list_ids: Vec<Uuid>,
    ) -> Result<usize, diesel::result::Error>;
    async fn get_lists_from_campaign(&self, c_id: Uuid) -> Result<Vec<ListResponse>, diesel::result::Error>;
}

pub struct CampaignListRepositoryImpl;

#[async_trait]
impl CampaignListRepository for CampaignListRepositoryImpl {
    async fn add_lists_to_campaign(
        &self,
        campaign_id: Uuid,
        list_ids: Vec<Uuid>,
    ) -> Result<Vec<NewListInCampaign>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        let new_lists: Vec<NewListInCampaign> = list_ids
            .into_iter()
            .map(|list_id| NewListInCampaign { campaign_id, list_id })
            .collect();

        diesel::insert_into(campaign_lists::table)
            .values(&new_lists)
            .returning((campaign_lists::campaign_id, campaign_lists::list_id))
            .get_results(&mut conn)
            .map(|results| {
                results
                    .into_iter()
                    .map(|(campaign_id, list_id)| NewListInCampaign { campaign_id, list_id })
                    .collect()
            })
    }

    async fn delete_lists_from_campaign(
        &self,
        c_id: Uuid,
        list_ids: Vec<Uuid>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::campaign_lists::dsl::*;

        let mut conn = get_connection_pool().await;

        diesel::delete(
            campaign_lists
                .filter(campaign_id.eq(c_id))
                .filter(list_id.eq_any(list_ids)),
        )
        .execute(&mut conn)
    }

    /// a function to get all the ids of the lists associated with the campaign id...
    /// takes camapaign id as an parameter...
    async fn get_lists_from_campaign(&self, c_id: Uuid) -> Result<Vec<ListResponse>, diesel::result::Error> {
        use crate::schema::campaign_lists::dsl::*;
        let mut conn = get_connection_pool().await;

        let result = campaign_lists
            .filter(campaign_id.eq(c_id))
            .inner_join(lists.on(list_id.eq(id)))
            .select((id, name, namespace_id, description, created_at, updated_at))
            .load::<(Uuid, String, Uuid, Option<String>, DateTime<Utc>, DateTime<Utc>)>(&mut conn)?
            .into_iter()
            .map(
                |(l_id, l_name, l_namespace_id, l_description, l_created_at, l_updated_at)| ListResponse {
                    id: l_id,
                    namespace_id: l_namespace_id,
                    name: l_name,
                    description: l_description,
                    created_at: l_created_at,
                    updated_at: l_updated_at,
                },
            )
            .collect::<Vec<ListResponse>>();

        Ok(result)
    }
}
