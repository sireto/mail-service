use crate::models::{campaign::Campaign, list::List};
use crate::schema::campaign_lists;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[primary_key(campaign_id, list_id)]
#[table_name = "campaign_lists"]
#[belongs_to(Campaign)] // relationship with Contact
#[belongs_to(List)] //relationship with List
pub struct ListInCampaign {
    pub campaign_id: Uuid,
    pub list_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ListInCampaign {
    pub fn campaign(&self, conn: &mut PgConnection) -> QueryResult<Campaign> {
        use crate::schema::campaigns::dsl::*;
        campaigns.filter(id.eq(self.campaign_id)).first(conn)
    }

    pub fn list(&self, conn: &mut PgConnection) -> QueryResult<List> {
        use crate::schema::lists::dsl::*;
        lists.filter(id.eq(self.list_id)).first(conn)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct AddListRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_ids: Vec<Uuid>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::campaign_lists)]
pub struct NewListInCampaign {
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub campaign_id: Uuid,
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_id: Uuid,
}
