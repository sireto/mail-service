use aws_config::Region;
use aws_sdk_sesv2::{
    error::SdkError,
    operation::send_email::SendEmailOutput,
    types::{Body, Content, Destination, EmailContent, Message, MessageHeader, SuppressionListReason},
    Client,
};

use crate::error::AppError;
use aws_sdk_sesv2::config::Credentials;
use std::{env, sync::Arc};

use crate::servers::{
    servers_repo,
    servers_services::{self, ServerServiceTrait},
};
pub async fn create_aws_client() -> Result<Client, AppError> {
    // Fetch IAM credentials from environment variables or any other source
    let access_key_id = env::var("AWS_ACCESS_KEY_ID")
        .map_err(|_| AppError::InternalServerError(Some("AWS_ACCESS_KEY_ID is not configured".to_string())))?;
    let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY")
        .map_err(|_| AppError::InternalServerError(Some("AWS_SECRET_ACCESS_KEY is not configured".to_string())))?;
    let session_token = env::var("AWS_SESSION_TOKEN").ok(); // Optional for temporary credentials

    // Create the credentials object
    let credentials = Credentials::new(
        &access_key_id,
        &secret_access_key,
        session_token,
        None,
        "custom_credentials",
    );

    // Region is configurable; the previous hardcoded ap-southeast-1 is kept as the default
    // so existing deployments behave identically.
    let region = Region::new(env::var("AWS_REGION").unwrap_or_else(|_| "ap-southeast-1".to_string()));

    // Create AWS configuration with the credentials and region
    let config = aws_config::from_env()
        .credentials_provider(credentials)
        .region(region)
        .load()
        .await;

    // Create and return the SES client
    Ok(Client::new(&config))
}

/**
 * a function to send mail using the AWS SES service...
 */
// from/to/cc/bcc/subject/body is inherently a wide signature; grouping it into a
// struct is a refactor for its own change, not this one.
#[allow(clippy::too_many_arguments)]
pub async fn send_mail(
    client: Client,
    from: &str,
    to: Vec<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    subject: &str,
    html_data: &str,
    mail_id: Option<&str>,
) -> Result<SendEmailOutput, SdkError<aws_sdk_sesv2::operation::send_email::SendEmailError>> {
    let mut destination = Destination::builder().build();
    destination.to_addresses = Some(to.clone());
    // Conditionally set `cc_addresses` if `cc` is provided...
    destination.cc_addresses = cc.filter(|cc_list| !cc_list.is_empty());

    // Conditionally set `bcc_addresses` if `bcc` is provided...
    destination.bcc_addresses = bcc.filter(|bcc_list| !bcc_list.is_empty());

    let subject_content = Content::builder().data(subject).charset("UTF-8").build()?;
    let html_content = Content::builder().data(html_data).charset("UTF-8").build()?;
    // let text_content = Content::builder()
    //     .data(payload.template_data)
    //     .charset("UTF-8")
    //     .build()
    //     .expect("Error while building text content");

    let body = Body::builder().html(html_content).build();

    // The `mailId` header is what the SNS webhook uses to correlate delivery events back
    // to a mail row. It is optional: one-off template sends have no mail row to correlate
    // to, and previously passing None here panicked and made that endpoint unusable.
    let headers = mail_id
        .map(|id| MessageHeader::builder().name("mailId").value(id).build())
        .transpose()?
        .map(|header| vec![header]);

    let msg = Message::builder()
        .subject(subject_content)
        .set_headers(headers)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    // Absent configuration set means no open/click tracking, which is a degraded feature
    // rather than a reason to fail the send.
    let configuration_set_name = env::var("AWS_SES_CONFIGURATION_SET_NAME").ok();

    client
        .send_email()
        .from_email_address(from)
        .destination(destination)
        .content(email_content)
        .set_configuration_set_name(configuration_set_name)
        .send()
        .await
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
pub async fn get_recent_bounces(
    client: &Client,
) -> Result<(), SdkError<aws_sdk_sesv2::operation::send_email::SendEmailError>> {
    let response = client
        .list_suppressed_destinations()
        .reasons(SuppressionListReason::Bounce)
        .send()
        .await;

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

pub async fn create_aws_client_db(server_id: &str) -> Result<Client, AppError> {
    let server_repo = Arc::new(servers_repo::ServerRepoImpl);
    let server_service = servers_services::ServerService::new(server_repo);

    let server = server_service.get_server_by_id(server_id).await?;

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
    Ok(Client::new(&config))
}
