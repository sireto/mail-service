use crate::models::contact::Contact;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::list_contacts;
use crate::models::list_contacts::NewContactInList;
use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};


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
pub trait ListContactRepository {
    async fn add_contacts_to_list(&self, list_id: Uuid, contact_id: Vec<Uuid>) -> Result<Vec<NewContactInList>, diesel::result::Error>;
    async fn delete_contacts_from_list(&self, list_id: Uuid, contact_id: Vec<Uuid>) -> Result<usize, diesel::result::Error>; 
    async fn get_contacts_from_lists(&self, lists: Vec<Uuid>) -> Result<Vec<Contact>, diesel::result::Error>;
}

pub struct ListContactRepositoryImpl;


#[async_trait]
impl ListContactRepository for ListContactRepositoryImpl {

 async fn add_contacts_to_list(
    &self,
    list_id: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<Vec<NewContactInList>, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    let new_contacts: Vec<NewContactInList> = contact_ids
        .into_iter()
        .map(|contact_id| NewContactInList { list_id, contact_id })
        .collect();

    diesel::insert_into(list_contacts::table)
        .values(&new_contacts)
        .returning((list_contacts::list_id, list_contacts::contact_id))
        .get_results(&mut conn)
        .map(|results| {
            results
                .into_iter()
                .map(|(list_id, contact_id)| NewContactInList { list_id, contact_id })
                .collect()
        })
}


 async fn delete_contacts_from_list(
    &self,
    listId: Uuid,
    contact_ids: Vec<Uuid>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::list_contacts::dsl::*;
    let mut conn = get_connection_pool().await;

    diesel::delete(
        list_contacts
            .filter(list_id.eq(listId))
            .filter(contact_id.eq_any(contact_ids))
    )
    .execute(&mut conn)
}
async fn get_contacts_from_lists(&self, lists: Vec<Uuid>) -> Result<Vec<Contact>, diesel::result::Error> {
    use crate::schema::{contacts, list_contacts};
    let mut conn = get_connection_pool().await;

    let result = contacts::table
        .inner_join(list_contacts::table.on(contacts::id.eq(list_contacts::contact_id)))
        .filter(list_contacts::list_id.eq_any(lists))
        .select(contacts::all_columns)
        .load::<Contact>(&mut conn)?;

    Ok(result)
}
}