use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::mails::dsl::*;
use diesel::prelude::*;
use chrono::{ Utc, DateTime };
use crate::models::mail::{
    Mail,
    NewMail,
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
    async fn create_mail(&self, payload: NewMail) -> Result<Mail, diesel::result::Error>;
    async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<Mail>, diesel::result::Error>;
    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error>;
    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<Mail, diesel::result::Error>;
    async fn delete_mail(&self, mail_id: String) -> Result<Mail, diesel::result::Error>;
}

pub struct MailRepositoryImpl;

#[async_trait]
impl MailRepository for MailRepositoryImpl {
    async fn create_mail(&self, payload: NewMail) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        diesel::insert_into(mails)
            .values(&payload)
            .returning(Mail::as_returning())
            .get_result::<Mail>(&mut conn)
    }

    async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<Mail>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        println!("\n\n\n THE VALUES OF CAMPAIGN IDS: {:?}, FROM: {:?}, TO: {:?} \n\n\n", campaign_ids, from, to);

        // Start the query...
        let mut query = mails.into_boxed();

        // Add filter to the campaign_ids if present...
        if !campaign_ids.is_none() {
            query = query.filter(campaign_id.eq(campaign_ids.unwrap()));
        }

        // Add filter to the from date if present...
        if let Some(from_date) = from {
            query = query.filter(sent_at.ge(from_date));
        }

        if let Some(to_date) = to {
            query = query.filter(sent_at.le(to_date));
        }

        // Execute the query and return
        let results = query.load::<Mail>(&mut conn)?;

        println!("\n\n\n NEW MAIL RESPONSES AFTER APPLYTING THE DATE FILTER {:?} \n\n\n", results);

        
        Ok(results)
        
    }

    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error> {
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

    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
            .set(status.eq(new_status))
            .get_result(&mut conn)
    }

    async fn delete_mail(&self, mail_id: String) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(mails.find(mail_id))
            .get_result(&mut conn)
    }
}