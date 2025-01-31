use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::mails::dsl::*;
use diesel::prelude::*;
use crate::models::mail::{
    Mail,
    CreateMailRequest,
    UpdateMailRequest,
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
pub trait MailRepository {
    async fn create_mail(&self, payload: CreateMailRequest) -> Result<Mail, diesel::result::Error>;
    async fn get_all_mails(&self) -> Result<Vec<Mail>, diesel::result::Error>;
    async fn update_mail(&self, mail_id: Uuid, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error>;
    async fn delete_mail(&self, mail_id: Uuid) -> Result<Mail, diesel::result::Error>;
}

pub struct MailRepositoryImpl;

#[async_trait]
impl MailRepository for MailRepositoryImpl {
    async fn create_mail(&self, payload: CreateMailRequest) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::insert_into(mails)
            .values(&payload)
            .returning(Mail::as_returning())
            .get_result::<Mail>(&mut conn)
    }

    async fn get_all_mails(&self) -> Result<Vec<Mail>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        mails
            .select((
                id,
                mail_message,
                contact_id,
                template_id,
                campaign_id,
                sent_at,
                status,
            ))
            .load::<Mail>(&mut conn)
    }

    async fn update_mail(&self, mail_id: Uuid, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
            .set((
                mail_message.eq(&payload.mail_message),
                template_id.eq(&payload.template_id),
                campaign_id.eq(&payload.campaign_id),
                status.eq(&payload.status.unwrap_or_default())
            ))
            .get_result(&mut conn)
    }

    async fn delete_mail(&self, mail_id: Uuid) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(mails.find(mail_id))
            .get_result(&mut conn)
    }
}