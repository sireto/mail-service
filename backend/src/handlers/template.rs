use crate::models::mail::CreateMailRequest;
use crate::models::template::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, SendMailRequest, SendMailResponse, TemplateResponse, UpdateTemplateRequest, UpdateTemplateResponse };
use serde_json::Value;
use uuid::Uuid;

use axum::{
    extract:: Path, Json, http::StatusCode
};

use crate::services::template_service;
use crate::handlers::mail as mail_handler;


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
    let templates_result = template_service::get_all_templates().await?;

    if templates_result.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No templates found".to_string()));
    }

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
pub async fn get_templates_by_id(Path(template_id): Path<String>) -> Result<Json<GetTemplateResponse>, (StatusCode, String)> {
    // Try to parse the template_id from String to Uuid
    let template_id = Uuid::parse_str(&template_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid template ID format".to_string()))?;

    // Fetch the template by ID from the service
    let template_result = template_service::get_template_by_id(template_id).await;

    match template_result {
        Ok(template) => Ok(Json(template)), // Return the template wrapped in Json
        Err((status, message)) => Err((status, message)), // Propagate error if not found
    }
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
) -> Result<Json<UpdateTemplateResponse>, (StatusCode, String)> {

    let update_template_response = template_service::update_template(template_id, payload).await?;

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
    let delete_template_response = template_service::delete_template(template_id).await?;

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
) -> Result<Json<SendMailResponse>, (StatusCode, String)> {
    let send_templated_email_response = template_service::send_templated_email(template_id, payload).await;

    match send_templated_email_response {
        Ok(response) => {
            let contact_id = Uuid::parse_str("c1746c77-6710-43dd-9312-75e5eabfd76c").unwrap();
            let payload = CreateMailRequest {
                mail_message: response.message.clone(),
                contact_id,
                template_id: Some(response.id),
                campaign_id: None,
                sent_at: response.sent_at,
                status: "pending".to_string(),
            };

            let mail_added_response = mail_handler::add_mail(Json(payload)).await;

            let _ = match mail_added_response {
                Ok(mail_response) => Ok(Json(mail_response)),
                Err((status, message)) => Err((status, message)),
            };

            Ok(Json(response))
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}