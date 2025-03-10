use std::sync::Arc;
use crate::models::template::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, ParseMjml2HtmlRequest, ParseMjml2HtmlResponse, SendMailRequest, SendMailResponse, Template, UpdateTemplateRequest, UpdateTemplateResponse };

use crate::repositories::template_repo::{self, TemplateRepository, TemplateRespositoryImpl};
use crate::services::aws_service;
use crate::utils::email_utils::enumerate_list;
use crate::utils::mjml_parser::mjml_to_html;
use uuid::Uuid;
use chrono::Utc;

use tera::{Context, Tera, Value};

use email_address::*;
use anyhow::{Error, Result};
use crate::error::AppError;


pub struct TemplateService {
    repository: Arc<dyn TemplateRepository + Send + Sync>
}

impl TemplateService {
    pub fn new(repository: Arc<dyn TemplateRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn get_template_by_id(&self, template_id: Uuid) -> Result<Template, diesel::result::Error> {
        self.repository.get_template_by_id(template_id).await
    }

    pub async fn get_all_templates(&self) -> Result<Vec<Template>, diesel::result::Error> {
        self.repository.get_all_templates().await
    }

    pub async fn create_template(&self, payload: CreateTemplateRequest) -> Result<Template, diesel::result::Error> {
        self.repository.create_template(payload).await
    }

    pub async fn update_template(&self, template_id: Uuid, payload: UpdateTemplateRequest) -> Result<Template, diesel::result::Error> {
        self.repository.update_template(template_id, payload).await
    }

    pub async fn delete_template(&self, template_id: Uuid) -> Result<Template, diesel::result::Error> {
        self.repository.delete_template(template_id).await
    }
}

pub async fn get_template_by_id(template_id: Uuid) -> Result<GetTemplateResponse, AppError> {
    // Call the repository function to get the template by ID...
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);
    let template = template_service.get_template_by_id(template_id).await?;

    Ok(GetTemplateResponse {
        id: template.id.to_string(),
        name: template.name,
        namespace_id: template.namespace_id.to_string(),
        template_data: template.template_data,
        content_plaintext: template.content_plaintext,
        content_html: template.content_html,
        created_at: template.created_at,
        updated_at: template.updated_at,
    })
}

pub async fn get_all_templates () -> Result<Vec<GetTemplateResponse>, AppError> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);

    let all_templates = template_service.get_all_templates().await?;

    Ok(all_templates.into_iter().map(|template| GetTemplateResponse {
        id: template.id.to_string(),
        name: template.name,
        namespace_id: template.namespace_id.to_string(),
        template_data: template.template_data,
        content_plaintext: template.content_plaintext,
        content_html: template.content_html,
        created_at: template.created_at,
        updated_at: template.updated_at
    }).collect())
}

pub async fn create_template (payload: CreateTemplateRequest) -> Result<CreateTemplateResponse, AppError> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);


    let new_template = CreateTemplateRequest {
        name: payload.name,
        namespace_id: payload.namespace_id,
        template_data: payload.template_data,
        content_plaintext: payload.content_plaintext,
        content_html: payload.content_html
    };

    let created_template = template_service.create_template(new_template).await?;

    Ok(CreateTemplateResponse {
        id: created_template.id.to_string(),
        name: created_template.name,
        created_at: created_template.created_at
    })
}

pub async fn update_template (
    template_id: Uuid,
    payload: UpdateTemplateRequest
) -> Result<UpdateTemplateResponse, AppError> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);

    let updated_template = template_service.update_template( template_id, payload).await?;

    Ok(UpdateTemplateResponse {
        id: updated_template.id,
        name: updated_template.name,
        updated_at: updated_template.updated_at
    })
}

pub async fn delete_template (
    template_id: Uuid,
) -> Result<DeleteTemplateResponse, AppError> {
    let template_repository = Arc::new(TemplateRespositoryImpl);
    let template_service = TemplateService::new(template_repository);

    let deleted_template = template_service.delete_template( template_id).await?;

    Ok(DeleteTemplateResponse {
        id: deleted_template.id,
        name: deleted_template.name,
    })
}

pub async fn send_templated_email(
    template_id: Uuid,
    payload: SendMailRequest,
) -> Result<SendMailResponse, AppError> {
    let client = aws_service::create_aws_client().await;

    // Validate receiver...
    if payload.receiver.clone().unwrap_or_default().trim().is_empty() && payload.cc.clone().unwrap_or_default().trim().is_empty() && payload.bcc.clone().unwrap_or_default().trim().is_empty() {
        return Err(AppError::BadRequestError(Some("No valid receivers found".to_string())));
    }

    // Fetch the template by ID...
    
    let template = get_template_by_id(template_id).await?;

    let parsed_html = populate_and_parse_template(&template, &payload).await.map_err(|e| AppError::InternalServerError(Some("Failed to parse mjml to html".to_string())))?;

    let (receiver_list, cc_list, bcc_list) = handle_receivers(&client, &payload).await.map_err(|e| AppError::InternalServerError(Some("Failed to extract the receivers".to_string())))?;

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
    .await?;

    Ok(SendMailResponse {
        id: template_id,
        message_id: result.message_id().unwrap_or_default().to_string(),
        name: template.name,
        to: receiver_list,
        from: payload.from,
        cc: cc_list,
        bcc: bcc_list,
        message: parsed_html,
        sent_at: Utc::now(),
    })
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

    let rendered = tera.render("demo_template", &context)?;

    println!("THE POPULATED HTML ====> {}", &rendered);

    let parsed_html = mjml_to_html(rendered)?;
    println!("----------THE RENDERED MJML PART ===> {}", &parsed_html);
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

    let emails = enumerate_list(receiver.to_string()).map_err(anyhow::Error::from)?;

    if !emails.is_empty() {
        return Ok(emails);
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

pub async fn parse_mjml_to_html(payload: ParseMjml2HtmlRequest) -> Result<ParseMjml2HtmlResponse, AppError> {
    let parsed_html = mjml_to_html(payload.mjml).map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    Ok(ParseMjml2HtmlResponse {
        html: parsed_html
    })
}