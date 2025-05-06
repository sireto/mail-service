use crate::servers::servers_model::ServerTypeEnum;
use crate::{ appState::DbPooledConnection, GLOBAL_APP_STATE };
use crate::schema::mails::dsl::*;
use crate::schema::contacts::dsl as contacts_dsl;
use crate::schema::bounce_logs::dsl as bounce_logs_dsl;
use crate::schema::campaign_senders::dsl as campaign_senders_dsl;
use crate::schema::campaigns::dsl as campaigns_dsl;
use crate::schema::servers::{dsl as servers_dsl, server_type};
use diesel::prelude::*;
use chrono::{ Utc, DateTime, Duration };
use crate::models::mail::{
    Mail, MailWithDetails, NewMail, UpdateMailRequest
};
use uuid::Uuid;
use diesel::dsl::sql;
use diesel::sql_types::{ Nullable, Text };
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
    async fn get_mails_by_status(&self, mail_status: &str, is_ascending: bool) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
    async fn get_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, diesel::result::Error>;
    async fn update_mail_attempts(&self, mail_id: String) -> Result<Mail, diesel::result::Error>;
    async fn udpate_mail_last_try_error(&self, mail_id: String, error: &str) -> Result<Mail, diesel::result::Error>;
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
                sql::<Nullable<Text>>("NULL")
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
                bounce_logs_dsl::reason.nullable(),
                sql::<Nullable<Text>>("NULL")
            ))
            .filter(contact_id.eq(c_id))
            .order(sent_at.desc())
            .load::<MailWithDetails>(&mut conn)
    }

    async fn get_mails_by_status(&self, mail_status: &str, is_ascending: bool) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        
        let mut query_results = mails
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .inner_join(campaigns_dsl::campaigns.on(campaign_id.eq(campaigns_dsl::id.nullable())))
            .inner_join(campaign_senders_dsl::campaign_senders.on(campaigns_dsl::campaign_senders.eq(campaign_senders_dsl::id.nullable())))
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
                campaign_senders_dsl::from_name.nullable(),
            ))
            .filter(status.eq(mail_status))
            .into_boxed();

            if is_ascending {
                query_results = query_results.order(sent_at.asc());
            } else {
                query_results = query_results.order(sent_at.desc());
            }
            query_results.load::<MailWithDetails>(&mut conn)
    }

    async fn get_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
        let stale_duration_minutes = 30; // only send retry those mails that are older than 30 minutes...

        mails
            .inner_join(contacts_dsl::contacts.on(contact_id.eq(contacts_dsl::id)))
            .left_outer_join(bounce_logs_dsl::bounce_logs.on(id.eq(bounce_logs_dsl::mail_id)))
            .left_outer_join(servers_dsl::servers.on(server_id.eq(servers_dsl::id.nullable())))
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
                sql::<Nullable<Text>>("NULL")
            ))
            .filter(status.eq("submitted"))
            .filter(server_type.eq(ServerTypeEnum::AWS))
            .filter(sent_at.lt(Utc::now().naive_utc() - Duration::minutes(stale_duration_minutes)))
            .filter(attempts.le(3))
            .order(sent_at.asc())
            .load::<MailWithDetails>(&mut conn)
    }

    async fn update_mail_attempts(&self, mail_id: String) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
            .set(attempts.eq(attempts + 1))
            .get_result(&mut conn)
    }

    async fn udpate_mail_last_try_error(&self, mail_id: String, error: &str) -> Result<Mail, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(mails.find(mail_id))
            .set(last_error.eq(error))
            .get_result(&mut conn)
    }
}