// the following functions are not currently used it might be used for the cleanup process for the contact list while developing...

use aws_sdk_sesv2::{Client, Error};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tera::{Context, Tera};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        contact::{Contact, CreateContactRequest},
        template::GetTemplateResponse,
    },
    repositories::{campaign_lists_repo::CampaignListRepositoryImpl, list_contact_repo::ListContactRepositoryImpl},
    services::{campaign_service::CampaignListService, list_service::ListContactService},
};

pub async fn delete_contact_from_list(client: &Client, list_name: &str, email: &str) -> Result<(), Error> {
    // Attempt to delete the contact from the specified contact list

    match client
        .delete_contact()
        .contact_list_name(list_name)
        .email_address(email)
        .send()
        .await
    {
        Ok(_) => {
            println!("Contact {} successfully deleted from list {}", email, list_name);
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to delete contact {} from list {}: {}", email, list_name, err);
            Err(err.into())
        }
    }
}

pub async fn populate_contact_template(
    template: &GetTemplateResponse,
    contact: &Contact,
) -> Result<String, anyhow::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("campaign_template", &template.content_html)
        .map_err(|e| anyhow::anyhow!("Template error: {}", e))?;

    // Use contact attributes as template data
    let mut context = Context::new();
    if let Some(Value::Object(map)) = &contact.attribute {
        for (key, value) in map {
            // `value.to_string()` on a JSON string keeps its quotes, so {{ city }} rendered
            // as "Tokyo" (with the quote characters) in the delivered email.
            match value {
                Value::String(text) => context.insert(key, text),
                Value::Null => context.insert(key, ""),
                other => context.insert(key, &other.to_string()),
            }
        }
    }

    // basic contact info...
    context.insert("first_name", &contact.first_name);
    context.insert("last_name", &contact.last_name);
    context.insert("email", &contact.email);

    let rendered = tera.render("campaign_template", &context)?;
    let parsed_template = mrml::parse(&rendered)?;
    let html_output = parsed_template.render(&mrml::prelude::render::RenderOptions::default())?;

    Ok(html_output)
}

// Helper function to parse CSV data
pub fn parse_csv_data(
    data: &[u8],
    namespace_id: Uuid,
    delimiter: &str,
    _mode: &str,
    _status: &str,
    _overwrite: bool,
) -> Result<Vec<CreateContactRequest>, String> {
    // `delimiter.as_bytes()[0]` panicked on an empty delimiter field, and silently ignored
    // a multi-byte one.
    let delimiter_byte = match delimiter.as_bytes() {
        [single] => *single,
        [] => b',',
        _ => return Err(format!("delimiter must be a single byte, got {delimiter:?}")),
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter_byte)
        .flexible(true)
        .from_reader(data);

    let mut contacts = Vec::new();
    let headers = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .clone();

    // Validate headers (must have at least 'email')
    if !headers.iter().any(|h| h.trim() == "email") {
        return Err("CSV file must contain an 'email' column".to_string());
    }

    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read CSV record: {}", e))?;

        // Find email, name, and attributes columns
        let email = record
            .iter()
            .zip(headers.iter())
            .find(|(_, h)| h.trim() == "email")
            .map(|(v, _)| v.trim())
            .ok_or_else(|| "Email not found in record".to_string())?;

        if email.is_empty() {
            continue; // Skip records with empty email
        }

        let name = record
            .iter()
            .zip(headers.iter())
            .find(|(_, h)| h.trim() == "name")
            .map(|(v, _)| v.trim())
            .unwrap_or("");

        let attributes = record
            .iter()
            .zip(headers.iter())
            .find(|(_, h)| h.trim() == "attributes")
            .map(|(v, _)| serde_json::from_str(v.trim()).unwrap_or(Value::Object(serde_json::Map::new())))
            .unwrap_or(Value::Object(serde_json::Map::new()));

        // Split name into first_name and last_name
        let (first_name, last_name) = split_name(name);

        // Create contact
        contacts.push(CreateContactRequest {
            namespace_id,
            email: email.to_string(),
            first_name,
            last_name,
            attribute: Some(attributes),
        });
    }

    Ok(contacts)
}

// Helper function to split full name into first_name and last_name
pub fn split_name(name: &str) -> (String, String) {
    let parts: Vec<&str> = name.split_whitespace().collect();
    match parts.len() {
        0 => (String::new(), String::new()),
        1 => (parts[0].to_string(), String::new()),
        _ => {
            let first_name = parts[0].to_string();
            let last_name = parts[1..].join(" ");
            (first_name, last_name)
        }
    }
}

pub async fn get_unique_contacts_from_campaign(campaign_id: Uuid) -> Result<Vec<Contact>, AppError> {
    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);

    let list_ids: Vec<Uuid> = campaign_list_service
        .get_lists_from_campaign(campaign_id)
        .await?
        .into_iter()
        .map(|list| list.id)
        .collect();

    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let list_contact_service = ListContactService::new(list_contact_repository);

    let all_contacts = list_contact_service.get_contacts_from_lists(list_ids).await?;

    // Deduplicate by email
    let mut unique_contacts = HashMap::new();
    for contact in all_contacts {
        unique_contacts.entry(contact.email.clone()).or_insert(contact);
    }

    Ok(unique_contacts.into_values().collect())
}
