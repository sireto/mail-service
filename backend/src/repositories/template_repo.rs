// data access layer for templates...
use crate::{appState::DbPooledConnection, GLOBAL_APP_STATE};
use crate::schema::templates::dsl::*;
use diesel::prelude::*;
use crate::models::template::{ CreateTemplateRequest, Template, UpdateTemplateRequest };
use uuid::Uuid;
use async_trait::async_trait;
use mockall::automock;

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}

#[automock]
#[async_trait]
pub trait TemplateRepository {
    async fn get_template_by_id(&self, template_id: Uuid) -> Result<Template, diesel::result::Error>;
    async fn get_all_templates (&self) -> Result<Vec<Template>, diesel::result::Error>;
    async fn update_template (
        &self,
        template_id: Uuid, 
        payload: UpdateTemplateRequest
    ) -> Result<Template, diesel::result::Error>;
    async fn delete_template (&self, template_id: Uuid) -> Result<Template, diesel::result::Error>;
    async fn create_template (
        &self,
        payload: CreateTemplateRequest
    ) -> Result<Template, diesel::result::Error>;
}

pub struct TemplateRespositoryImpl;

#[async_trait]
impl TemplateRepository for TemplateRespositoryImpl {
    async fn get_template_by_id(&self, template_id: Uuid) -> Result<Template, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        // Use `find()` to get the template by id
        templates
            .filter(id.eq(template_id))
            .first(&mut conn)  // Fetch the first matching result
    }
    
    async fn get_all_templates (&self) -> Result<Vec<Template>, diesel::result::Error> {
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
    
    async fn create_template (
        &self,
        payload: CreateTemplateRequest
    ) -> Result<Template, diesel::result::Error> {
    
        let mut conn = get_connection_pool().await;
    
        diesel::insert_into(templates)
            .values(&payload)
            .returning(Template::as_returning())
            .get_result::<Template>(&mut conn)
    }
    
    async fn update_template (
            &self,
            template_id: Uuid, 
            payload: UpdateTemplateRequest
        ) -> Result<Template, diesel::result::Error> {
    
        let mut conn = get_connection_pool().await;
        
        diesel::update(templates.find(template_id))
            .set((
                name.eq(payload.name),
                template_data.eq(payload.template_data),
                content_html.eq(payload.content_html),
                content_plaintext.eq(payload.content_plaintext),
                updated_at.eq(diesel::dsl::now),
            ))
            .get_result(&mut conn)
    }
    
    async fn delete_template (&self, template_id: Uuid) -> Result<Template, diesel::result::Error> {
        let mut conn = get_connection_pool().await;
    
        diesel::delete(templates.filter(id.eq(template_id)))
            .get_result(&mut conn)
    }
}
