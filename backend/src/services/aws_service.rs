use aws_sdk_sesv2::{error::SdkError, operation::send_email::SendEmailOutput, types::{Body, Content, Destination, EmailContent, Message, BulkEmailEntry, SuppressionListReason }, Client };
use aws_config::{BehaviorVersion, Region};

use std::{env, error::Error, sync::Arc};
use aws_sdk_sesv2::config::Credentials;

use crate::servers::{servers_repo, servers_services::{self, ServerServiceTrait}};
pub async fn create_aws_client() -> Client {

    
    // Fetch IAM credentials from environment variables or any other source
    let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
    let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");
    let session_token = env::var("AWS_SESSION_TOKEN").ok(); // Optional for temporary credentials

    // Create the credentials object
    let credentials = Credentials::new(
        &access_key_id,
        &secret_access_key,
        session_token,
        None, 
        "custom_credentials",
    );

    // Specify the region
    let region = Region::new("ap-southeast-1");

    // Create AWS configuration with the credentials and region
    let config = aws_config::from_env()
        .credentials_provider(credentials)
        .region(region)
        .load()
        .await;

    // Create and return the SES client
    Client::new(&config)
}

/**
 * a function to send mail using the AWS SES service...
 */
pub async fn send_mail(
    client: Client, 
    from: &str, 
    to: Vec<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>, 
    subject: &str, 
    html_data: &str
) -> Result<SendEmailOutput, SdkError<aws_sdk_sesv2::operation::send_email::SendEmailError>> {
    let mut destination = Destination::builder().build();
    destination.to_addresses = Some(to.clone());
    // Conditionally set `cc_addresses` if `cc` is provided...
    destination.cc_addresses = if let Some(cc_list) = cc {
        if !cc_list.is_empty() {
            Some(cc_list)
        } else {
            None
        }
    } else {
        None
    };

    // Conditionally set `bcc_addresses` if `bcc` is provided...
    destination.bcc_addresses = if let Some(bcc_list) = bcc {
        if !bcc_list.is_empty() {
            Some(bcc_list)
        } else {
            None
        }
    } else {
        None
    };

    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let html_content = Content::builder()
        .data(html_data)
        .charset("UTF-8")
        .build()
        .expect("Error while building html Content");
    // let text_content = Content::builder()
    //     .data(payload.template_data)
    //     .charset("UTF-8")
    //     .build()
    //     .expect("Error while building text content");

    let body = Body::builder().html(html_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    let result = client
        .send_email()
        .from_email_address(from)
        .destination(destination)
        .content(email_content)
        .send()
        .await;

    result
}

//Currnetly this is not being used 
pub async fn send_bulk_email(
    client: &aws_sdk_sesv2::Client,
    from_email: &str,
    entries: Vec<aws_sdk_sesv2::types::BulkEmailEntry>,
) -> Result<(), anyhow::Error> {
    client.send_bulk_email()
        .from_email_address(from_email)
        .set_bulk_email_entries(Some(entries)) // Use setter for Option<Vec>
        .set_configuration_set_name(Some("Mail-Service-SNS".to_string()))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("AWS SES error: {}", e))?;

    Ok(())
}

/// a function to get the recent aws bounces...
pub async fn get_recent_bounces(client: &Client) -> Result<(), SdkError<aws_sdk_sesv2::operation::send_email::SendEmailError>> {
    let response = client.list_suppressed_destinations().reasons(SuppressionListReason::Bounce).send().await;

    match response {
        Ok(result) => {
            // Handle the Option<&[SuppressedDestinationSummary]>
            let suppressed_list = result.suppressed_destination_summaries();
            
            for suppressed in suppressed_list {
                println!("Suppressed Email: {:?}", suppressed.email_address);
            }
        }
        Err(err) => {
            eprintln!("Error fetching suppressed emails: {:?}", err);
        }
    }

    Ok(())
}

pub async fn create_aws_client_db(server_id: &str) -> Client {
    let server_repo = Arc::new(servers_repo::ServerRepoImpl);
    let server_service = servers_services::ServerService::new(server_repo);

    let server = server_service.get_server_by_id(&server_id).await.unwrap();

    let mut access_key_id = String::new();
    let mut secret_access_key = String::new();
    let mut region_str = String::new();

    if let Some(aws_creds) = server.aws_credentials {
        // Parse the JSON credentials, handle potential missing keys
        if let Some(key) = aws_creds.get("access_key_id") {
            access_key_id = key.as_str().unwrap_or("").trim_matches('"').to_string();
        }
        
        if let Some(secret) = aws_creds.get("secret_access_key") {
            secret_access_key = secret.as_str().unwrap_or("").trim_matches('"').to_string();
        }
        if let Some(reg) = aws_creds.get("region") {
            region_str = reg.as_str().unwrap_or("").trim_matches('"').to_string(); 
        }
    }
    
    let session_token = env::var("AWS_SESSION_TOKEN").ok(); // Optional for temporary credentials

    // Create the credentials object
    let credentials = Credentials::new(
        &access_key_id,
        &secret_access_key,
        session_token,
        None, 
        "custom_credentials",
    );

    let region = Region::new(region_str.to_string());

    // Create AWS configuration with the credentials and region
    let config = aws_config::from_env()
        .credentials_provider(credentials)
        .region(region)
        .load()
        .await;

    // Create and return the SES client
    Client::new(&config)
}