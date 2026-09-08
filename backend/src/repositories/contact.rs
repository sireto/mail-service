use crate::models::contact::{Contact, CreateContactRequest, UpdateContactRequest};
use crate::models::list::List;
use crate::models::list_contacts::ListContact;
use crate::schema::contacts::dsl::*;
use crate::{app_state::DbPooledConnection, GLOBAL_APP_STATE};
use async_trait::async_trait;
use diesel::dsl::now;
use diesel::prelude::*;
#[cfg(feature = "mocks")]
use mockall::automock;
use uuid::Uuid;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait ContactRepository {
    async fn create_contacts(&self, payloads: Vec<CreateContactRequest>)
        -> Result<Vec<Contact>, diesel::result::Error>;
    /// Returns one page of contacts plus the total number matching the filters.
    async fn get_all_contacts(
        &self,
        namespace: Uuid,
        list_id: Option<Uuid>,
        search: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Contact>, i64), diesel::result::Error>;
    async fn update_contact(
        &self,
        contact_id: Uuid,
        payload: UpdateContactRequest,
    ) -> Result<Contact, diesel::result::Error>;
    async fn delete_contact(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
    async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error>;
    async fn get_contact_by_email(
        &self,
        namespace: Uuid,
        contact_email: String,
    ) -> Result<Contact, diesel::result::Error>;
    async fn get_list_contacts(&self, contact_ids: Vec<Uuid>) -> Result<Vec<ListContact>, diesel::result::Error>;
    async fn get_lists_by_ids(&self, list_ids: Vec<Uuid>) -> Result<Vec<List>, diesel::result::Error>;
    async fn upsert_contacts(
        &self,
        payloads: Vec<CreateContactRequest>,
        overwrite: bool,
    ) -> Result<Vec<Contact>, diesel::result::Error>;
}

pub struct ContactRepositoryImpl;

#[async_trait]
impl ContactRepository for ContactRepositoryImpl {
    async fn create_contacts(
        &self,
        payloads: Vec<CreateContactRequest>,
    ) -> Result<Vec<Contact>, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::insert_into(contacts)
            .values(&payloads)
            .returning(Contact::as_returning())
            .get_results::<Contact>(&mut conn)
    }

    async fn get_all_contacts(
        &self,
        namespace: Uuid,
        list_id: Option<Uuid>,
        search: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Contact>, i64), diesel::result::Error> {
        use crate::schema::contacts::dsl::*;
        use diesel::prelude::*;

        let mut conn = get_connection_pool().await;

        // Same filters, counted. Boxed queries cannot be cloned, so this is applied twice
        // rather than shared.
        let mut count_query = contacts
            .select(diesel::dsl::count_star())
            .filter(namespace_id.eq(namespace))
            .into_boxed();

        if let Some(list_id_val) = list_id {
            use crate::schema::list_contacts::dsl as lc;
            count_query = count_query.filter(
                id.eq_any(
                    lc::list_contacts
                        .select(lc::contact_id)
                        .filter(lc::list_id.eq(list_id_val)),
                ),
            );
        }

        if let Some(search_term) = search.clone() {
            let pattern = format!("%{}%", search_term.to_lowercase());
            count_query = count_query.filter(
                first_name
                    .ilike(pattern.clone())
                    .or(last_name.ilike(pattern.clone()))
                    .or(email.ilike(pattern)),
            );
        }

        let total: i64 = count_query.first(&mut conn)?;

        // Contacts are scoped to a namespace. Without this filter every namespace saw every
        // other namespace's subscribers.
        let mut query = contacts
            .select((
                id,
                namespace_id,
                first_name,
                last_name,
                email,
                attribute,
                created_at,
                updated_at,
            ))
            .filter(namespace_id.eq(namespace))
            .into_boxed();

        // Filter by list_id if present
        if let Some(list_id_val) = list_id {
            use crate::schema::list_contacts::dsl::*;
            query = query.filter(id.eq_any(list_contacts.select(contact_id).filter(list_id.eq(list_id_val))));
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

        // A deterministic order is what makes offset paging stable; email is unique within
        // the namespace, so it is a safe tiebreaker.
        let items = query
            .order((created_at.desc(), email.asc()))
            .limit(limit)
            .offset(offset)
            .load::<Contact>(&mut conn)?;

        Ok((items, total))
    }

    async fn update_contact(
        &self,
        contact_id: Uuid,
        payload: UpdateContactRequest,
    ) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::update(contacts.find(contact_id))
            .set((
                first_name.eq(&payload.first_name),
                last_name.eq(&payload.last_name),
                email.eq(&payload.email),
                attribute.eq(&payload.attribute),
                updated_at.eq(now),
            ))
            .get_result(&mut conn)
    }

    async fn delete_contact(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        diesel::delete(contacts.filter(id.eq(contact_id))).get_result(&mut conn)
    }

    async fn get_contact_by_id(&self, contact_id: Uuid) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        contacts.filter(id.eq(contact_id)).first(&mut conn)
    }

    async fn get_contact_by_email(
        &self,
        namespace: Uuid,
        contact_email: String,
    ) -> Result<Contact, diesel::result::Error> {
        let mut conn = get_connection_pool().await;

        // An email is only unique within a namespace now, so the namespace is part of the
        // lookup key rather than an afterthought.
        contacts
            .filter(namespace_id.eq(namespace))
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
        lists.filter(id.eq_any(list_ids)).load::<List>(&mut conn)
    }

    async fn upsert_contacts(
        &self,
        payloads: Vec<CreateContactRequest>,
        overwrite: bool,
    ) -> Result<Vec<Contact>, diesel::result::Error> {
        use diesel::pg::upsert::excluded;
        let mut conn = get_connection_pool().await;

        if overwrite {
            diesel::insert_into(contacts)
                .values(&payloads)
                // Matches contacts_namespace_id_email_key, which replaced the global unique
                // constraint on email.
                .on_conflict((namespace_id, email))
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
                // Matches contacts_namespace_id_email_key, which replaced the global unique
                // constraint on email.
                .on_conflict((namespace_id, email))
                .do_nothing()
                .returning(Contact::as_returning())
                .get_results(&mut conn)
        }
    }
}
