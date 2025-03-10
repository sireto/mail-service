use crate::error::AppError;
use crate::models::mail::CreateMailRequest;
use crate::models::template::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, ParseMjml2HtmlRequest, ParseMjml2HtmlResponse, SendMailRequest, SendMailResponse, TemplateResponse, UpdateTemplateRequest, UpdateTemplateResponse };
use serde_json::Value;
use uuid::Uuid;

use axum::{
    extract:: Path, Json, http::StatusCode
};

use crate::services::template_service;
use crate::handlers::mail_handler as mail_handler;
use crate::utils::template_utils;


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
pub async fn get_templates() -> Result<Json<Vec<GetTemplateResponse>>, AppError> {
    let templates_result = template_service::get_all_templates().await?;

    Ok(Json(templates_result))
}

#[utoipa::path(
    get,
    path = "/api/templates/{template_id}",
    responses(
        (status = 200, description = "Get Template By ID", body = TemplateResponse),
        (status = 404)
    )
)]
pub async fn get_templates_by_id(Path(template_id): Path<String>) -> Result<Json<GetTemplateResponse>, AppError> {
    // Try to parse the template_id from String to Uuid
    let template_id = Uuid::parse_str(&template_id)?;

    // Fetch the template by ID from the service
    let template_result = template_service::get_template_by_id(template_id).await?;

    Ok(Json(template_result))
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
) -> Result<Json<CreateTemplateResponse>, AppError> {

    let create_new_template = template_service::create_template(payload).await?;

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
) -> Result<Json<UpdateTemplateResponse>, AppError> {

    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&template_id)?;

    let update_template_response = template_service::update_template(uuid_id, payload).await?;

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
) -> Result<Json<DeleteTemplateResponse>, AppError> {
    let uuid_id = Uuid::parse_str(&template_id)?;

    let delete_template_response = template_service::delete_template(uuid_id).await?;

    Ok(Json(delete_template_response))
}

#[utoipa::path(
    post,
    path = "/api/templates/{template_id}/send",
    params(
        ("template_id" = String, Path, description = "ID of the template to send")
    ),
    responses(
        (status = 200, description = "Template sent successfully", body = TemplateResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn send_templated_email(
    Path(template_id): Path<String>,
    Json(payload): Json<SendMailRequest>
) -> Result<Json<SendMailResponse>, AppError> {
    let template_uuid_id = Uuid::parse_str(&template_id)?;

    let send_templated_email_response = template_service::send_templated_email(template_uuid_id, payload.clone())
        .await?;

    let emails = template_utils::merge_receipients(
        payload.receiver.unwrap_or("".to_string()), 
        payload.cc.unwrap_or("".to_string()), 
        payload.bcc.unwrap_or("".to_string())
    );

    let payload = CreateMailRequest {
        id: send_templated_email_response.id.to_string(),
        mail_message: send_templated_email_response.message.clone(),
        email: emails,
        template_id: Some(send_templated_email_response.id),
        campaign_id: None,
        sent_at: send_templated_email_response.sent_at,
        status: "pending".to_string(),
    };

    mail_handler::add_mail(Json(payload)).await;

    Ok(Json(send_templated_email_response))
}

#[utoipa::path(
    post,
    path = "/api/templates/parse-mjml",
    responses(
        (status = 200, description = "Template sent successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn parse_mjml_to_html(
    Json(payload): Json<ParseMjml2HtmlRequest>
) -> Result<Json<ParseMjml2HtmlResponse>, AppError> {
    let parsed_html = template_service::parse_mjml_to_html(payload).await?;
    
    Ok(Json(parsed_html))
}