use std::sync::Arc;
use crate::{prisma::template, schema::{DeleteTemplateResponse, TemplateResponse, UpdateTemplateRequest, UpdateTemplateResponse, GetTemplateResponse}};

use chrono::DateTime;

use axum::{
    extract:: {
        Path, State
    }, Json, http::StatusCode
};

use crate::appState::AppState;

#[utoipa::path(
    get,
    path = "/api/templates",
    responses(
        (status = 200, description = "List of templates", body = Vec<TemplateResponse>),
        (status = 404, description = "No templates found")
    )
)]
pub async fn get_templates(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<GetTemplateResponse>>, (StatusCode, String)> {
    let prisma = &state.db;

    let templates = prisma.template()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // if templates.is_empty() {
    //     return Err((StatusCode::NOT_FOUND, "No templates found".to_string()));
    // }

    let response = templates
        .into_iter()
        .map(|template| GetTemplateResponse {
            id: template.id,
            name: template.name,
            namespace_id: template.namespace_id,
            template_data: template.template_data,
            content_plaintext: template.content_plaintext,
            content_html: template.content_html,
            created_at: DateTime::from(template.created_at),
            updated_at: DateTime::from(template.updated_at),
        })
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/templates",
    responses(
        (status = 200, description = "Create a template", body = ()),
        (status = 404)
    )
)]
pub async fn create_template() {}


#[utoipa::path(
    patch,
    path = "/api/templates/{template_id}",
    params(
        ("template_id" = String, Path, description = "ID of the template to update")
    ),
    responses(
        (status = 200, description = "Template updated successfully", body = UpdateTemplateResponse),
        (status = 404)
    )
)]
pub async fn update_template(
    State(state): State<Arc<AppState>>, 
    Path(template_id): Path<String>, 
    Json(payload): Json<UpdateTemplateRequest>
    ) -> Result<Json<UpdateTemplateResponse>, (StatusCode, String)> {
    let prisma = &state.db;
    
    if payload.name.is_none() && payload.content_html.is_none() && payload.content_plaintext.is_none() && payload.template_data.is_none() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "At least one field (name, content_html, content_plaintext, template_data) is required".to_string()));
    }

    // Build the update operations dynamically...
    let mut updates: Vec<template::SetParam> = Vec::new();
     
    if let Some(name) = payload.name {
        updates.push(template::name::set(name));
    }
    if let Some(content_html) = payload.content_html {
        updates.push(template::content_html::set(content_html));
    }
    if let Some(content_plaintext) = payload.content_plaintext {
        updates.push(template::content_plaintext::set(Some(content_plaintext)));
    }
    if let Some(template_data) = payload.template_data {
        updates.push(template::template_data::set(template_data));
    }


    let response = prisma.template().update(
        template::id::equals(template_id.to_string()),
        updates
        // template::name::set(payload.name),

    )
    .exec()
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let update_response = UpdateTemplateResponse {
        id: response.id,
        name: response.name,
        updated_at: DateTime::from(response.updated_at),
    };

    Ok(Json(update_response))
}


#[utoipa::path(
    delete,
    path = "/api/templates/{template_id}",
    params(
        ("template_id" = String, Path, description = "ID of the template to delete")
    ),
    responses(
        (status = 200, description = "Template deleted successfully", body = DeleteTemplateResponse),
        (status = 404)
    )
)]
pub async fn delete_template(
    State(state): State<Arc<AppState>>,
    Path(template_id): Path<String>
) -> Result<Json<DeleteTemplateResponse>, (StatusCode, String)> {
    let prisma = &state.db;

    let response = prisma.template()
    .delete(template::id::equals(template_id.to_string()))
    .exec()
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    println!("The deleted template is: {response:?}");

    let delete_response = DeleteTemplateResponse {
        id: response.id,
        name: response.name,
    };

    Ok(Json(delete_response))
}