use crate::models::{contact::Contact, list::List};
use crate::schema::list_contacts;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{app_state::DbPooledConnection, GLOBAL_APP_STATE};

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

impl ContactInList {
    pub fn contact(&self, conn: &mut PgConnection) -> QueryResult<Contact> {
        use crate::schema::contacts::dsl::*;
        contacts.filter(id.eq(self.contact_id)).first(conn)
    }

    pub fn list(&self, conn: &mut PgConnection) -> QueryResult<List> {
        use crate::schema::lists::dsl::*;
        lists.filter(id.eq(self.list_id)).first(conn)
    }
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[diesel(primary_key(list_id, contact_id))]
#[diesel(table_name = list_contacts)]
#[diesel(belongs_to(Contact))] // relationship with Contact
#[diesel(belongs_to(List))] //relationship with List
pub struct ContactInList {
    pub list_id: Uuid,
    pub contact_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct AddContactRequest {
    #[schema(value_type = String, example = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_ids: Vec<Uuid>, // Collect contact IDs to add to the list
}
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::list_contacts)]
pub struct NewContactInList {
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_id: Uuid,
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::list_contacts)]
pub struct ListContact {
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub list_id: Uuid,
    #[schema(value_type=String, example="a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")]
    pub contact_id: Uuid,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}
