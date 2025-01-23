// data access layer for templates...
use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use crate::schema::templates::dsl::*;
use diesel::prelude::*;
use crate::model::{ CreateTemplateRequest, Template, UpdateTemplateRequest };
use uuid::Uuid;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

pub async fn get_all_templates () -> Result<Vec<Template>, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    templates 
        .select((
            id,
            namespace_id,
            name,
            template_data,
            content_plaintext,
            content_html,
            created_at,
            updated_at,
        )) // Select columns explicitly
        .load::<Template>(&mut conn)
}

pub async fn create_template (
    payload: CreateTemplateRequest
) -> Result<Template, diesel::result::Error> {

    let mut conn = get_connection_pool().await;

    diesel::insert_into(templates)
        .values(&payload)
        .returning(Template::as_returning())
        .get_result::<Template>(&mut conn)
}

pub async fn update_template (
        template_id: Uuid, 
        payload: UpdateTemplateRequest
    ) -> Result<Template, diesel::result::Error> {

    let mut conn = get_connection_pool().await;
    
    diesel::update(templates.find(template_id))
        .set((
            name.eq(payload.name),
            template_data.eq(payload.template_data),
            content_html.eq(payload.content_html),
            content_plaintext.eq(payload.content_plaintext)
        ))
        .get_result(&mut conn)
}

pub async fn delete_template (template_id: Uuid) -> Result<Template, diesel::result::Error> {
    let mut conn = get_connection_pool().await;

    diesel::delete(templates.filter(id.eq(template_id)))
        .get_result(&mut conn)
}