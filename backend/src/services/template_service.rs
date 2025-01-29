use crate::models::template::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, SendMailRequest, SendMailResponse, UpdateTemplateRequest, UpdateTemplateResponse };

use crate::repositories::template_repo;
use crate::repositories::template_repo::{self, TemplateRepository, TemplateRespositoryImpl};
use crate::schema::namespaces::created_at;
use crate::services::aws_service;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use tera::{Context, Tera, Value};

use axum::http::StatusCode;


pub async fn get_template_by_id(namespace_id:Uuid, template_id: Uuid) -> Result<GetTemplateResponse, (StatusCode, String)> {
    // Call the repository function to get the template by ID
    let template = template_repo::get_template_by_id(namespace_id, template_id).await;

    match template {
        Ok(template) => {
            // Map the template to the response format
            let response_template = GetTemplateResponse {
                id: template.id.to_string(),
                name: template.name,
                namespace_id: template.namespace_id.to_string(),
                template_data: template.template_data,
                content_plaintext: template.content_plaintext,
                content_html: template.content_html,
                created_at: template.created_at,
                updated_at: template.updated_at,
            };

            Ok(response_template)
        },
        Err(err) => {
            // Return an error if the template was not found
            Err((StatusCode::NOT_FOUND, err.to_string()))
        }
    }
}

pub async fn get_all_templates(namespace_id: Uuid) -> Result<Vec<GetTemplateResponse>, (StatusCode, String)> {
    let all_templates = template_repo::get_all_templates(namespace_id).await;

    let response_templates = match all_templates {
        Ok(templates) => templates.into_iter().map(|template| GetTemplateResponse {
            id: template.id.to_string(),
            name: template.name,
            namespace_id: template.namespace_id.to_string(),
            template_data: template.template_data,
            content_plaintext: template.content_plaintext,
            content_html: template.content_html,
            created_at: template.created_at,
            updated_at: template.updated_at
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_templates)
}

pub async fn create_template (payload: CreateTemplateRequest) -> Result<CreateTemplateResponse, (StatusCode, String)> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);


    let new_template = CreateTemplateRequest {
        name: payload.name,
        namespace_id: payload.namespace_id,
        template_data: payload.template_data,
        content_plaintext: payload.content_plaintext,
        content_html: payload.content_html
    };

    let created_template = template_service.create_template(new_template).await;

    let response_template = match created_template {
        Ok(template) => CreateTemplateResponse {
            id: template.id.to_string(),
            name: template.name,
            created_at: template.created_at
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_template)
}

pub async fn update_template(
    namespace_id: String,
    template_id: String,
    payload: UpdateTemplateRequest
) -> Result<UpdateTemplateResponse, (StatusCode, String)> {

    let uuid_namespace_id = Uuid::parse_str(&namespace_id)
    .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid namespace UUID format".to_string()))?;
    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&template_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let updated_template = template_repo::update_template( uuid_namespace_id,uuid_id, payload).await;

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

pub async fn delete_template(
    namespace_id: String,
    template_id: String,
) -> Result<DeleteTemplateResponse, (StatusCode, String)> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);
    // Convert 'template_id' (String) to 'Uuid'...
    let uuid_id = Uuid::parse_str(&template_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

        let uuid_namespace_id = Uuid::parse_str(&namespace_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let deleted_template = template_repo::delete_template( uuid_namespace_id,uuid_id).await;

    let response_template = match deleted_template {
        Ok(template) => DeleteTemplateResponse {
            id: template.id,
            name: template.name,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_template)
}

pub async fn send_templated_email(
    namespace_id: String,
    template_id: String,
    payload: SendMailRequest,
) -> Result<SendMailResponse, anyhow::Error> {
    let client = aws_service::create_aws_client().await;

    let uuid_namespace_id = Uuid::parse_str(&namespace_id).unwrap();

    // Validate receiver...
    if payload.receiver.clone().unwrap_or_default().trim().is_empty() && payload.cc.clone().unwrap_or_default().trim().is_empty() && payload.bcc.clone().unwrap_or_default().trim().is_empty() {
        return Err(anyhow::anyhow!("Receiver email address is required"));
    }

    // Fetch the template by ID...
    let template_uuid_id = Uuid::parse_str(&template_id)
        .map_err(|_| anyhow::anyhow!("Invalid template ID".to_string()))?;
    let template = get_template_by_id(uuid_namespace_id,template_uuid_id).await.map_err(|e| anyhow::anyhow!(e.1))?;

    let parsed_html = populate_and_parse_template(&template, &payload).await?;

    let (receiver_list, cc_list, bcc_list) = handle_receivers(&client, &payload).await?;

    // Send email
    let result = aws_service::send_mail(
        client,
        &payload.from,
        receiver_list.clone(),
        Some(cc_list.clone()),
        Some(bcc_list.clone()),
        &payload.subject,
        &parsed_html,
    )
    .await;

    match result {
        Ok(_) => Ok(SendMailResponse {
            id: Uuid::new_v4(),
            name: template.name,
            to: receiver_list,
            from: payload.from,
            cc: cc_list,
            bcc: bcc_list
        }),
        Err(err) => {
            eprintln!("Failed to send templated email: {}", err);
            Err(anyhow::anyhow!(format!("Failed to send email: {:?}", err)).into())
        }
    }
}

/// a function to populate and parse the template...
async fn populate_and_parse_template(template: &GetTemplateResponse, payload: &SendMailRequest) -> Result<String, Error> {
    // Render template
    let mut tera = Tera::default();
    tera.add_raw_template("demo_template", &template.content_html)
        .map_err(|e| anyhow::anyhow!(format!("Failed to load template: {e}")))?;

    let parsed_data: Value = serde_json::from_str(&payload.template_data)
        .map_err(|e| anyhow::anyhow!(format!("Invalid template data: {e}")))?;

    let mut context = Context::new();
    if let Some(map) = parsed_data.as_object() {
        for (key, value) in map {
            if let Some(value_str) = value.as_str() {
                context.insert(key, value_str);
            }
        }
    }

    let rendered = tera.render("demo_template", &context)
        .map_err(|e| anyhow::anyhow!(format!("Failed to render template: {e}")))?;

    let parsed_template_html = mrml::parse(&rendered)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse MJML template: {e}")))?;

    let opts = mrml::prelude::render::Options::default();
    let parsed_html = parsed_template_html
        .render(&opts)
        .map_err(|e| anyhow::anyhow!(format!("Failed to render MJML to HTML: {e}")))?;

    Ok(parsed_html)
}

/// Helper function to handler the receivers (either to, cc, or bc)...
async fn handle_receivers(
    client: &aws_sdk_sesv2::Client,
    payload: &SendMailRequest,
) -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    // Helper function to process optional receivers...
    async fn process_optional_receivers(
        client: &aws_sdk_sesv2::Client,
        receiver: &Option<String>,
    ) -> Result<Vec<String>> {
        match receiver {
            Some(value) if !value.trim().is_empty() => process_receivers(client, value).await,
            _ => Ok(vec![]), // Return an empty list if None or empty...
        }
    }

    // Process all receiver lists...
    let receiver_list = process_optional_receivers(client, &payload.receiver).await?;
    let cc_list = process_optional_receivers(client, &payload.cc).await?;
    let bcc_list = process_optional_receivers(client, &payload.bcc).await?;

    // Ensure at least one recipient is present...
    if receiver_list.is_empty() && cc_list.is_empty() && bcc_list.is_empty() {
        return Err(anyhow::anyhow!("No valid receivers found"));
    }

    Ok((receiver_list, cc_list, bcc_list))
}

// a helper function to extract the receivers (either to, cc, or bcc) on its respective list...
async fn process_receivers(client: &aws_sdk_sesv2::Client, receiver: &str) -> Result<Vec<String>> {
    // Check if the receiver is a single valid email...
    if EmailAddress::is_valid(receiver) {
        return Ok(vec![receiver.to_string()]);
    }

    // Check if the receiver is a comma-separated list of emails...
    let receiver_list: Vec<String> = receiver
        .split(',')
        .map(|r| r.trim().to_string())
        .collect();

    // Validate each email in the list...
    if receiver_list.iter().all(|email| EmailAddress::is_valid(email)) {
        return Ok(receiver_list);
    }

    // Treat `receiver` as a contact list name...
    let resp = client
        .list_contacts()
        .contact_list_name(receiver)
        .send()
        .await?;

    let contacts = resp.contacts();
    let email_addresses: Vec<String> = contacts
        .iter()
        .filter_map(|contact| contact.email_address().map(|e| e.to_string()))
        .collect();

    if email_addresses.is_empty() {
        anyhow::bail!("No valid email addresses found in the contact list '{}'", receiver);
    }

    Ok(email_addresses)
}
