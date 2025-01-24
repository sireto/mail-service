use crate::model::{ CreateTemplateRequest, CreateTemplateResponse, DeleteTemplateResponse, GetTemplateResponse, SendMailRequest, SendMailResponse, UpdateTemplateRequest, UpdateTemplateResponse };

use crate::repositories::template_repo::{self, get_template_by_id};
use crate::services::aws_service;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use aws_sdk_sesv2::Error;

use tera::{Context, Tera, Value};

use axum::http::StatusCode;


pub async fn get_template_by_id_service(template_id: Uuid) -> Result<GetTemplateResponse, (StatusCode, String)> {
    // Call the repository function to get the template by ID
    let template = template_repo::get_template_by_id(template_id).await;

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
                created_at: DateTime::from_naive_utc_and_offset(template.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(template.updated_at, Utc),
            };

            Ok(response_template)
        },
        Err(err) => {
            // Return an error if the template was not found
            Err((StatusCode::NOT_FOUND, err.to_string()))
        }
    }
}

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

pub async fn send_templated_email_service (
    template_id: String,
    payload: SendMailRequest
) -> Result<SendMailResponse, Error> {
    let client = aws_service::create_aws_client().await;

    let template_uuid_id = Uuid::parse_str(&template_id).unwrap();

    let template = get_template_by_id(template_uuid_id).await.unwrap();

    let mut tera = Tera::default();
    tera.add_raw_template("demo_template", &template.content_html).unwrap();

    let template_data = payload.template_data;
    println!("Template data value: {:?}", template_data);

    let json_string = template_data
        .replace("'", "\"") // Replace single quotes with double quotes
        .replace("{", "{\"") // Add opening quotes to keys
        .replace(":", "\":") // Add closing quotes to keys
        .replace(", ", ", \""); // Add quotes to subsequent keys if needed
        // .replace(" ", ""); // Remove spaces...

    // Parse the JSON string into a `serde_json::Value`
    let parsed_data: Value = serde_json::from_str(&json_string).expect("Failed to parse data into JSON");

    println!("THe parsed data value: {:?}", parsed_data);

    // Create the Tera context and populate it with parsed data...
    let mut context = Context::new();

    if let Some(map) = parsed_data.as_object() {
        for (key, value) in map.iter() {
            if let Some(value_str) = value.as_str() {
                context.insert(key, value_str);
            }
        }
    }

    let rendered = tera.render("demo_template", &context).expect("Failed to render template");

    let parsed_template_html = mrml::parse(&rendered).unwrap_or_else(|_| panic!("Failed to parse mjml template"));

    let opts = mrml::prelude::render::Options::default();
    let parsed_template_html_from_mjml = parsed_template_html
        .render(&opts)
        .expect("Failed to render MJML to HTML");

    let subject = "Welcome";

    // contact lists data...
    let resp = client
        .list_contacts()
        .contact_list_name(payload.list)
        .send()
        .await?;

    let contacts = resp.contacts();
    let cs: Vec<String> = contacts 
        .iter()
        .map(|i| i.email_address().unwrap_or_default().to_string())
        .collect();

    let result = aws_service::send_mail(client, payload.from.clone(), cs.clone(), subject, &parsed_template_html_from_mjml).await;

    match result {
        Ok(_) => {
            println!("Templated email result: {:?}", result);
            let mail_response = SendMailResponse {
                id: Uuid::new_v4(),
                name: template.name.clone(),
                to: cs,
                from: payload.from
            };
            Ok(mail_response)
        }
        Err(err) => {
            eprintln!("Failed to send templated email: {}", err);
            Err(err.into())
        }
    }
}