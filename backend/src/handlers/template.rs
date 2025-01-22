use std::sync::Arc; 
use chrono::{DateTime, Utc};

use uuid::Uuid;
use axum::{
    extract:: {
        Path, State
    }, Json, http::StatusCode
};
use diesel::prelude::*;
use serde_json::Value;

use crate::schema::template::dsl::*;
// use crate::schema::template::dsl::*;
use crate::model::{GetTemplateResponse, CreateTemplateRequest, CreateTemplateResponse, Template};
use crate::appState::AppState;




pub async fn health_checker() -> Result<Json<Value>, (StatusCode, String)> {
    const MESSAGE: &str = "This endpoint is working fine";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok(Json(json_response))
}

#[utoipa::path(
    get,
    path = "/api/templates/{template_id}",
    params(
        ("template_id" = String, Path, description = "Template ID to fetch")
    ),
    responses(
        (status = 200, description = "Template found", body = GetTemplateResponse),
        (status = 404, description = "Template not found"),
        (status = 400, description = "Invalid template ID"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_template_by_id(
    State(state): State<Arc<AppState>>,
    Path(template_id): Path<String>,
) -> Result<Json<GetTemplateResponse>, (StatusCode, String)> {
    // Parse the template_id string to UUID
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid template ID: {}", e))
    })?;

    let conn = &mut state.db_pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Query the database for the specific template
    let template_result = template
        .select((
            id,
            namespace_id,
            name,
            template_data,
            content_plaintext,
            content_html,
            created_at,
            updated_at,
        ))
        .filter(id.eq(template_uuid))
        .first::<Template>(conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => {
                (StatusCode::NOT_FOUND, "Template not found".to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        })?;

    // Convert the template to the response format
    let response = GetTemplateResponse {
        id: template_result.id.to_string(),
        name: template_result.name,
        namespace_id: template_result.namespace_id.to_string(),
        template_data: template_result.template_data,
        content_plaintext: template_result.content_plaintext,
        content_html: template_result.content_html,
        created_at: DateTime::from_naive_utc_and_offset(template_result.created_at, Utc),
        updated_at: DateTime::from_naive_utc_and_offset(template_result.updated_at, Utc),
    };

    Ok(Json(response))
}


#[utoipa::path(
    get,
    path = "/api/templates",
    responses(
        (status = 200, description = "List of templates", body = Vec<GetTemplateResponse>),
        (status = 404, description = "No templates found")
    )
)]
pub async fn get_templates(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<GetTemplateResponse>>, (StatusCode, String)> {


    let conn = &mut state.db_pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Use Diesel to fetch templates from the database
    let templates_result = template // Use `templates::table` explicitly as the query source
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
        .load::<Template>(conn)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    if templates_result.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No templates found".to_string()));
    }

    // Map the templates to the response format
    let response: Vec<GetTemplateResponse> = templates_result
        .into_iter()
        .map(|templatee| GetTemplateResponse {
            id: templatee.id.to_string(),
            name: templatee.name,
            namespace_id: templatee.namespace_id.to_string(),
            template_data: templatee.template_data,
            content_plaintext: templatee.content_plaintext,
            content_html: templatee.content_html,
            created_at: DateTime::from_naive_utc_and_offset(templatee.created_at, Utc), // Convert NaiveDateTime to DateTime<Utc>
            updated_at: DateTime::from_naive_utc_and_offset(templatee.updated_at, Utc),
        })
        .collect();

    Ok(Json(response))
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
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTemplateRequest>,
) -> Result<Json<CreateTemplateResponse>, (StatusCode, String)> {
    let new_template = CreateTemplateRequest {
        namespace_id: payload.namespace_id,
        name: payload.name,
        template_data: payload.template_data,
        content_plaintext: payload.content_plaintext,
        content_html: payload.content_html
    };

    let conn = &mut state.db_pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error".to_string()))?;

    let create_new_template = diesel::insert_into(template)
        .values(&new_template)
        .returning(Template::as_returning())
        .get_result::<Template>(conn)
        .expect("Error creating new template");

    let create_response = CreateTemplateResponse {
        id: create_new_template.id.to_string(),
        name: create_new_template.name,
        created_at: DateTime::from_naive_utc_and_offset(create_new_template.created_at, Utc),
    };

    Ok(Json(create_response))
}

