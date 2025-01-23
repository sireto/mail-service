use crate::model::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, UpdateTemplateRequest, UpdateTemplateResponse };

use crate::repositories::template_repo;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use axum:: {
    extract::Json, http::StatusCode
};

pub async fn get_all_templates_service () -> Result<Vec<GetTemplateResponse>, (StatusCode, String)> {
    let all_templates = template_repo::get_all_templates().await;

    let response_templates = match all_templates {
        Ok(templates) => templates.into_iter().map(|template| GetTemplateResponse {
            id: template.id.to_string(),
            name: template.name,
            namespace_id: template.namespace_id.to_string(),
            template_data: template.template_data,
            content_plaintext: template.content_plaintext,
            content_html: template.content_html,
            created_at: DateTime::from_naive_utc_and_offset(template.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(template.updated_at, Utc),
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_templates)
}

pub async fn create_template_service (payload: CreateTemplateRequest) -> Result<CreateTemplateResponse, (StatusCode, String)> {
    let new_template = CreateTemplateRequest {
        name: payload.name,
        namespace_id: payload.namespace_id,
        template_data: payload.template_data,
        content_plaintext: payload.content_plaintext,
        content_html: payload.content_html
    };

    let created_template = template_repo::create_template(new_template).await;

    let response_template = match created_template {
        Ok(template) => CreateTemplateResponse {
            id: template.id.to_string(),
            name: template.name,
            created_at: DateTime::from_naive_utc_and_offset(template.created_at, Utc),
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_template)
}

pub async fn update_template_service (
    template_id: String,
    payload: UpdateTemplateRequest
) -> Result<UpdateTemplateResponse, (StatusCode, String)> {
    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&template_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let updated_template = template_repo::update_template( uuid_id, payload).await;

    let response_template = match updated_template {
        Ok(template) => UpdateTemplateResponse {
            id: template.id,
            name: template.name,
            updated_at: template.updated_at,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_template)
}

pub async fn delete_template_service (
    template_id: String,
) -> Result<DeleteTemplateResponse, (StatusCode, String)> {
    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&template_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let deleted_template = template_repo::delete_template( uuid_id).await;

    let response_template = match deleted_template {
        Ok(template) => DeleteTemplateResponse {
            id: template.id,
            name: template.name,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_template)
}