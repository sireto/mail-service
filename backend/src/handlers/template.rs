use crate::model::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, TemplateResponse, UpdateTemplateRequest, UpdateTemplateResponse, GetTemplateResponse };
use serde_json::Value;

use axum::{
    extract:: Path, Json, http::StatusCode
};

use crate::services::template_service;


#[derive(Clone, Debug, PartialEq)]
pub enum TemplateField {
    Name(String),
    ContentHtml(String),
    ContentPlainText(Option<String>),
    TemplateData(Value),
}

#[utoipa::path(
    get,
    path = "/api/templates",
    responses(
        (status = 200, description = "List of templates", body = Vec<TemplateResponse>),
        (status = 404)
    )
)]
pub async fn get_templates() -> Result<Json<Vec<GetTemplateResponse>>, (StatusCode, String)> {

    // Use Diesel to fetch templates from the database
    let templates_result = template_service::get_all_templates_service().await?;

    if templates_result.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No templates found".to_string()));
    }

    Ok(Json(templates_result))
}

#[utoipa::path(
    post,
    path = "/api/templates",
    responses(
        (status = 200, description = "Create a template", body = CreateTemplateResponse),
        (status = 404)
    )
)]
pub async fn create_template(
    Json(payload): Json<CreateTemplateRequest>,
) -> Result<Json<CreateTemplateResponse>, (StatusCode, String)> {

    let create_new_template = template_service::create_template_service(payload).await?;

    let create_response = CreateTemplateResponse {
        id: create_new_template.id.to_string(),
        name: create_new_template.name,
        created_at: create_new_template.created_at,
    };

    Ok(Json(create_response))
}

#[utoipa::path(
    patch,
    path = "/api/templates/{template_id}",
    params(
        ("template_id" = String, Path, description = "ID of the template to update")
    ),
    responses(
        (status = 200, description = "Template updated successfully", body = UpdateTemplateResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_template(
    
    Path(template_id): Path<String>,
    Json(payload): Json<UpdateTemplateRequest>
) -> Result<Json<UpdateTemplateResponse>, (StatusCode, String)> {

    let update_template_response = template_service::update_template_service(template_id, payload).await?;

    Ok(Json(update_template_response))
}

#[utoipa::path(
    delete,
    path = "/api/templates/{template_id}",
    params(
        ("template_id" = String, Path, description = "ID of the template to delete")
    ),
    responses(
        (status = 200, description = "Template deleted successfully", body = DeleteTemplateResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_template(
    Path(template_id): Path<String>
) -> Result<Json<DeleteTemplateResponse>, (StatusCode, String)> {
    let delete_template_response = template_service::delete_template_service(template_id).await?;

    Ok(Json(delete_template_response))
}