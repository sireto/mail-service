use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::mails::dsl::*;
use crate::schema::contacts::dsl as contacts_dsl;
use crate::schema::bounce_logs::dsl as bounce_logs_dsl;
use diesel::prelude::*;
use chrono::{ Utc, DateTime };
use crate::models::mail::{
    Mail, MailWithDetails, NewMail, UpdateMailRequest
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
    async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error>;
    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<Mail, diesel::result::Error>;
    async fn delete_mail(&self, mail_id: String) -> Result<Mail, diesel::result::Error>;
    async fn increment_mail_clicks(&self, mail_id: String) -> Result<Mail, diesel::result::Error>;
    async fn get_mails_by_contact(&self, c_id: Uuid) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
    async fn get_queued_mails(&self) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
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

    async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        let mut query = mails
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .left_outer_join(bounce_logs_dsl::bounce_logs.on(id.eq(bounce_logs_dsl::mail_id)))
            .select((
                id,
                mail_message,
                template_id,
                campaign_id,
                server_id,
                sent_at,
                status,
                open,
                clicks,
                scheduled_at,
                attempts,
                last_error,
                contacts_dsl::email,
                bounce_logs_dsl::reason.nullable(),
            ))
            .into_boxed();

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

        // Order most recent first...
        query = query.order(sent_at.desc());

        // Execute the query and return
        let results = query.load::<MailWithDetails>(&mut conn)?;

        
        Ok(results)
    }

    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
        .set(payload)
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

    async fn increment_mail_clicks(&self, mail_id: String) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
            .set(clicks.eq(clicks + 1))
            .get_result(&mut conn)
    }

    async fn get_mails_by_contact(&self, c_id: Uuid) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        // get all the mails for the contact...
        mails
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .left_outer_join(bounce_logs_dsl::bounce_logs.on(id.eq(bounce_logs_dsl::mail_id)))
            .select((
                id,
                mail_message,
                template_id,
                campaign_id,
                server_id,
                sent_at,
                status,
                open,
                clicks,
                scheduled_at,
                attempts,
                last_error,
                contacts_dsl::email,
                bounce_logs_dsl::reason.nullable()
            ))
            .filter(contact_id.eq(c_id))
            .order(sent_at.desc())
            .load::<MailWithDetails>(&mut conn)
    }

    async fn get_queued_mails(&self) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        mails
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .left_outer_join(bounce_logs_dsl::bounce_logs.on(id.eq(bounce_logs_dsl::mail_id)))
            .select((
                id,
                mail_message,
                template_id,
                campaign_id,
                server_id,
                sent_at,
                status,
                open,
                clicks,
                scheduled_at,
                attempts,
                last_error,
                contacts_dsl::email,
                bounce_logs_dsl::reason.nullable(),
            ))
            .filter(status.eq("queued"))
            .order(sent_at.asc())   // Order by sent_at ascending here...
            .load::<MailWithDetails>(&mut conn)
    }
}