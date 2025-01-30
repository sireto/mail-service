use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::contacts::dsl::*;
use diesel::prelude::*;
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
    async fn create_contact(&self, payload: CreateContactRequest) -> Result<Contact, diesel::result::Error>;
    async fn get_all_contacts(&self) -> Result<Vec<Contact>, diesel::result::Error>;
    async fn update_contact(&self, contact_id: Uuid, payload: UpdateContactRequest
    ) -> Result<Contact, diesel::result::Error>;
    async fn delete_contact(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
    async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
}

pub struct ContactRepositoryImpl;

#[async_trait]
impl ContactRepository for ContactRepositoryImpl {
    async fn create_contact(&self, payload: CreateContactRequest) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(contacts)
            .values(&payload)
            .returning(Contact::as_returning())
            .get_result::<Contact>(&mut conn)
    }

    async fn get_all_contacts(&self) -> Result<Vec<Contact>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        contacts
            .select((
                id,
                first_name,
                last_name,
                email,
                attribute,
                created_at,
                updated_at,
            ))
            .load::<Contact>(&mut conn)
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
}

// #[automock]
// pub async fn create_contact (
//     payload: CreateContactRequest
// ) -> Result<Contact, diesel::result::Error> {
//     let mut conn = get_connection_pool().await;

//     diesel::insert_into(contacts)
//         .values(&payload)
//         .returning(Contact::as_returning())
//         .get_result::<Contact>(&mut conn)
// }

// pub async fn get_all_contacts() -> Result<Vec<Contact>, diesel::result::Error> {
//     let mut conn = get_connection_pool().await;
    
//     contacts
//         .select((
//             id,
//             first_name,
//             last_name,
//             email,
//             attribute,
//             created_at,
//             updated_at,
//         ))
//         .load::<Contact>(&mut conn)
// }