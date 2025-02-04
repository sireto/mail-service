// the following functions are not currently used it might be used for the cleanup process for the contact list while developing...

use aws_sdk_sesv2::{ Client, Error };
use tera::{Context, Tera};
use serde_json::Value;

use crate::models::{contact::Contact, template::GetTemplateResponse};

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
            eprintln!(
                "Failed to delete contact {} from list {}: {}",
                email, list_name, err
            );
            Err(err.into())
        }
    }
}


async fn show_contacts(client: &Client, list: &str) -> Result<(), Error> {
    let resp = client
        .list_contacts()
        .contact_list_name(list)
        .send()
        .await?;

    println!("Contacts:");

    for contact in resp.contacts() {
        println!("  {}", contact.email_address().unwrap_or_default());
    }

    Ok(())
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
    if let Some(attrs) = &contact.attribute {
        if let Value::Object(map) = attrs {
            for (key, value) in map {
                context.insert(key, &value.to_string());
            }
        }
    }

    // basic contact info
    context.insert("first_name", &contact.first_name);
    context.insert("last_name", &contact.last_name);
    context.insert("email", &contact.email);

    let rendered = tera.render("campaign_template", &context)?;
    let parsed_template = mrml::parse(&rendered)?;
    let html_output = parsed_template.render(&mrml::prelude::render::Options::default())?;

    Ok(html_output)
}