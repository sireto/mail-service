use crate::models::list::List;
use crate::models::list_contacts::ListContact;
use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::contacts::dsl::*;
use diesel::prelude::*;
use diesel::dsl::now;
use crate::models::contact::{
    Contact,
    CreateContactRequest,
    UpdateContactRequest
};
use uuid::Uuid;
use mockall::{ automock, predicate::* };
use async_trait::async_trait;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[automock]
#[async_trait]
pub trait ContactRepository {
    async fn create_contacts(&self, payloads: Vec<CreateContactRequest>) -> Result<Vec<Contact>, diesel::result::Error>;
    async fn get_all_contacts(&self, list_id: Option<Uuid>, search: Option<String>) -> Result<Vec<Contact>, diesel::result::Error>;
    async fn update_contact(&self, contact_id: Uuid, payload: UpdateContactRequest
    ) -> Result<Contact, diesel::result::Error>;
    async fn delete_contact(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
    async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
    async fn get_contact_by_email(&self, contact_email: String) -> Result<Contact, diesel::result::Error>;
    async fn get_list_contacts(&self, contact_ids: Vec<Uuid>) -> Result<Vec<ListContact>, diesel::result::Error>;
    async fn get_lists_by_ids(&self, list_ids: Vec<Uuid>) -> Result<Vec<List>, diesel::result::Error>;
    async fn upsert_contacts(&self, payloads: Vec<CreateContactRequest>, overwrite: bool) -> Result<Vec<Contact>, diesel::result::Error>;
}

pub struct ContactRepositoryImpl;

#[async_trait]
impl ContactRepository for ContactRepositoryImpl {
    async fn create_contacts(&self, payloads: Vec<CreateContactRequest>) -> Result<Vec<Contact>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        diesel::insert_into(contacts)
            .values(&payloads)
            .returning(Contact::as_returning())
            .get_results::<Contact>(&mut conn)
    }

     async fn get_all_contacts(
        &self,
        list_id: Option<Uuid>,
        search: Option<String>,
    ) -> Result<Vec<Contact>, diesel::result::Error> {
        use crate::schema::contacts::dsl::*;
        use diesel::prelude::*;
    
        let mut conn = get_connection_pool().await;
    
        // Start building the query
        let mut query = contacts
            .select((id, first_name, last_name, email, attribute, created_at, updated_at))
            .into_boxed();
    
        // Filter by list_id if present
        if let Some(list_id_val) = list_id {
            use crate::schema::list_contacts::dsl::*;
            query = query.filter(
                id.eq_any(
                    list_contacts
                        .select(contact_id)
                        .filter(list_id.eq(list_id_val)),
                ),
            );
        }
    
        // Apply search filter
        if let Some(search_term) = search {
            let like_pattern = format!("%{}%", search_term.to_lowercase());
            let pattern = like_pattern.clone(); 
        
            query = query.filter(
                first_name
                    .ilike(pattern.clone())
                    .or(last_name.ilike(pattern.clone()))
                    .or(email.ilike(pattern)),
            );
        }
        
    
        query.load::<Contact>(&mut conn)
    }
    
    async fn update_contact (
        &self,
        contact_id: Uuid,
        payload: UpdateContactRequest
    ) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        diesel::update(contacts.find(contact_id))
            .set((
                first_name.eq(&payload.first_name),
                last_name.eq(&payload.last_name),
                email.eq(&payload.email),
                attribute.eq(&payload.attribute),
                updated_at.eq(now)
            ))
            .get_result(&mut conn)
    }

    async fn delete_contact (&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::delete(contacts.filter(id.eq(contact_id))
        ).get_result(&mut conn)
    }

    async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        contacts
            .filter(id.eq(contact_id))
            .first(&mut conn)
    }

    async fn get_contact_by_email(&self, contact_email: String) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        contacts
            .filter(email.eq(contact_email))
            .first(&mut conn)
    }
    async fn get_list_contacts(&self, contact_ids: Vec<Uuid>) -> Result<Vec<ListContact>, diesel::result::Error> {
        use crate::schema::list_contacts::dsl::*;
        let mut conn = get_connection_pool().await;
        list_contacts
            .filter(contact_id.eq_any(contact_ids))
            .load::<ListContact>(&mut conn)
    }

    async fn get_lists_by_ids(&self, list_ids: Vec<Uuid>) -> Result<Vec<List>, diesel::result::Error> {
        use crate::schema::lists::dsl::*;
        let mut conn = get_connection_pool().await;
        lists
            .filter(id.eq_any(list_ids))
            .load::<List>(&mut conn)
    }

    async fn upsert_contacts(
        &self,
        payloads: Vec<CreateContactRequest>,
        overwrite: bool
    ) -> Result<Vec<Contact>, diesel::result::Error> {
        use diesel::pg::upsert::excluded;
        let mut conn = get_connection_pool().await;
    
        
        let result = if overwrite {
            diesel::insert_into(contacts)
                .values(&payloads)
                .on_conflict(email)
                .do_update()
                .set((
                    first_name.eq(excluded(first_name)),
                    last_name.eq(excluded(last_name)),
                    attribute.eq(excluded(attribute)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Contact::as_returning())
                .get_results(&mut conn)
        } else {
            diesel::insert_into(contacts)
                .values(&payloads)
                .on_conflict(email)
                .do_nothing()
                .returning(Contact::as_returning())
                .get_results(&mut conn)
        };
    
        result
    }
}