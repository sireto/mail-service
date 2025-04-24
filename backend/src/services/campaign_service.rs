use crate::{error::AppError, models::{campaign::{CampaignSendResponse, DeleteCampaignResponse, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse, AddMailToQueueResponse}, campaign_lists::NewListInCampaign, mail::CreateMailRequest}, repositories::{campaign::{self, CampaginRepositoryImpl, CampaignRepository}, campaign_lists_repo::{CampaignListRepository, CampaignListRepositoryImpl}, list_contact_repo::ListContactRepositoryImpl, mail_repository::MailRepositoryImpl}, servers::{servers_handler::get_server_by_id, servers_model::{Server, ServerTypeEnum}, servers_repo::{self, ServerRepoImpl}, servers_services::{self, ServerService, ServerServiceTrait}}, services::mail_service::update_mail_status, utils::contact_lists_functions::{get_unique_contacts_from_campaign, populate_contact_template}};
use uuid::Uuid;
use std::{collections::HashSet, sync::Arc, env};
use axum::http::StatusCode;
use crate::models::campaign::{
    Campaign, 
    CreateCampaignRequest, 
    CreateCampaignResponse,
    ExtendedCreateCampaignRequest,
};
use crate::models::list::ListResponse;
use aws_sdk_sesv2::types::{builders::{BodyBuilder, ContentBuilder}, Body, Content, Destination, EmailContent, Message};
use crate::services::{aws_service, list_service::ListContactService, template_service::get_template_by_id};
use anyhow::{anyhow, Result};

use super::{aws_service::create_aws_client_db, campaign_sender_service::get_campaign_sender_by_id, mail_service::MailService };
use crate::services::mail_service as mail_service;


pub struct CampaignService {
    repository: Arc<dyn CampaignRepository + Send +Sync>
}

pub struct CampaignListService {
    repository: Arc<dyn CampaignListRepository + Send + Sync>
}

impl CampaignListService {
    pub fn new(repository: Arc<dyn CampaignListRepository + Send + Sync>) -> Self {
        Self {repository}
    }

    pub async fn add_lists_to_campaign(&self, campaign_id: Uuid, list_ids: Vec<Uuid>) -> Result<Vec<NewListInCampaign>, diesel::result::Error> {
        self.repository.add_lists_to_campaign(campaign_id, list_ids).await
    }

    async fn delete_lists_from_campaign(&self, list_id: Uuid, campaign_id: Vec<Uuid>) -> Result<usize, diesel::result::Error> {
        self.repository.delete_lists_from_campaign(list_id, campaign_id).await
    }

    pub async fn get_lists_from_campaign(&self, campaign_id: Uuid) -> Result<Vec<ListResponse>, diesel::result::Error> {
        self.repository.get_lists_from_campaign(campaign_id).await
    }
}

impl CampaignService {
    pub fn new(repository: Arc<dyn CampaignRepository + Send + Sync>) -> Self {
        Self {repository}
    }
    pub async fn create_campaign(&self, payload: CreateCampaignRequest) -> Result<Campaign, diesel::result::Error> {
        self.repository.create_campaign(payload).await
    }
    pub async fn get_all_campaigns(&self) -> Result<Vec<Campaign>, diesel::result::Error> {
        self.repository.get_all_campaigns().await
    }
    pub async fn get_campaign_by_id(&self, campaign_id: Uuid)-> Result<Campaign, diesel::result::Error> {
        self.repository.get_campaign_by_id(campaign_id).await
    }
    pub async fn update_campaign(&self, campaign_id: Uuid, payload: UpdateCampaignRequest) -> Result<Campaign, diesel::result::Error> {
        self.repository.update_campaign(campaign_id, payload).await
    }
    pub async fn delete_campaign(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error>{
        self.repository.delete_campaign(campaign_id).await
    }
}



pub async fn create_campaign( payload: ExtendedCreateCampaignRequest) -> Result<CreateCampaignResponse, AppError> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let new_campaign = CreateCampaignRequest {
        campaign_name: payload.base.campaign_name,
        template_id: payload.base.template_id,
        namespace_id: payload.base.namespace_id,
        status: payload.base.status,
        campaign_senders: payload.base.campaign_senders,
        scheduled_at: payload.base.scheduled_at,
    };

    let response = campaign_service.create_campaign(new_campaign).await?;

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);

    let list_ids: Vec<Uuid> = payload
        .list_ids
        .iter()
        .map(|id| Uuid::parse_str(id).expect("Invalid UUID format"))
        .collect();

    campaign_list_service.add_lists_to_campaign(response.id, list_ids).await?;

    Ok(CreateCampaignResponse {
        id: response.id, 
        campaign_name: response.campaign_name, 
        template_id: response.template_id, 
        namespace_id: response.namespace_id, 
        status: response.status, 
        campaign_senders: response.campaign_senders, 
        scheduled_at: response.scheduled_at, 
        created_at: response.created_at, 
        updated_at: response.updated_at
    })
}

pub async fn get_all_campaigns() -> Result<Vec<GetCampaignResponse>, AppError> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let all_campaigns = campaign_service.get_all_campaigns().await?;

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    
    let mut responses = Vec::new();
    for campaign in all_campaigns {
        let lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;
        responses.push(GetCampaignResponse {
            id: campaign.id,
            campaign_name: campaign.campaign_name,
            template_id: campaign.template_id,
            namespace_id: campaign.namespace_id,
            status: campaign.status,
            campaign_senders: campaign.campaign_senders,
            scheduled_at: campaign.scheduled_at,
            created_at: campaign.created_at,
            updated_at: campaign.updated_at,
            lists
        });
    }

    Ok(responses)
}

pub async fn get_campaign_by_id(campaign_id: Uuid) -> Result<GetCampaignResponse, AppError> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let campaign = campaign_service.get_campaign_by_id(campaign_id).await;

    let campaign = campaign.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;
    
    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    let lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let campaign_response = GetCampaignResponse{
        id: campaign.id, 
        campaign_name: campaign.campaign_name, 
        template_id: campaign.template_id, 
        namespace_id: campaign.namespace_id, 
        status: campaign.status, 
        campaign_senders: campaign.campaign_senders, 
        scheduled_at: campaign.scheduled_at, 
        created_at: campaign.created_at, 
        updated_at: campaign.updated_at,
        lists
    };

    Ok(campaign_response)
}

pub async fn update_campaign(campaign_id: Uuid, payload: UpdateCampaignRequest) -> Result<UpdateCampaignResponse, AppError> {
    
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    
    let updated_campaign_response = campaign_service.update_campaign(campaign_id, payload.clone()).await?;

    // get all the existing lists from the table...
    let existing_lists = campaign_list_service.get_lists_from_campaign(updated_campaign_response.id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    // convert existing list IDs from database to HashSet...
    let existing_list_ids: HashSet<Uuid> = existing_lists.iter().map(|list| list.id).collect();

    // Convert incoming list IDs from arguments to HashSet...
    let new_list_ids: HashSet<Uuid> = payload.list_ids.clone()
        .iter()
        .map(|id| *id)
        .collect();

    // lists to delete from the junction table...
    let to_delete: Vec<Uuid> = existing_list_ids
    .difference(&new_list_ids)
    .cloned()
    .collect();

    // lists to add to the junction table...
    let to_add: Vec<Uuid> = new_list_ids
    .difference(&existing_list_ids)
    .cloned()
    .collect();

    // Delete lists that are no longer linked to the updated_campaign_response
    if !to_delete.is_empty() {
    campaign_list_service
        .delete_lists_from_campaign(updated_campaign_response.id, to_delete.clone())
        .await?;
    }

    // Add new lists to the updated_campaign_response
    if !to_add.is_empty() {
        campaign_list_service
            .add_lists_to_campaign(updated_campaign_response.id, to_add.clone())
            .await?;
    }

    let lists = campaign_list_service.get_lists_from_campaign(updated_campaign_response.id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;


    // separate the above lists into two groups where the lists from the arguments are present in the list and the lists from the arguments are not present in the list...

    let response = UpdateCampaignResponse {
        id: updated_campaign_response.id, 
        campaign_name: updated_campaign_response.campaign_name, 
        template_id:  updated_campaign_response.template_id, 
        namespace_id: updated_campaign_response.namespace_id, 
        status: updated_campaign_response.status, 
        campaign_senders: updated_campaign_response.campaign_senders, 
        scheduled_at: Some(updated_campaign_response.scheduled_at), 
        updated_at: Some(updated_campaign_response.updated_at),
        lists
    };

    Ok(response)
}

pub async fn delete_campaign(campaign_id: Uuid)->Result<DeleteCampaignResponse, AppError> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let deleted_campaign_response = campaign_service.delete_campaign(campaign_id).await?;

    Ok(DeleteCampaignResponse {
        id: deleted_campaign_response.id, 
        campaign_name: deleted_campaign_response.campaign_name, 
        status: deleted_campaign_response.status
    })
}

pub async fn send_campaign_email(
    campaign_id: Uuid,
) -> Result<AddMailToQueueResponse, AppError> {
  let campaign = get_campaign_by_id(campaign_id).await?;
  let campaign_sender_id = campaign
    .campaign_senders
    .ok_or(AppError::NotFoundError(Some("Campaign sender not found".into())))?
    .to_string();


  let campaign_sender = get_campaign_sender_by_id(campaign_sender_id).await?;
  let server_id = campaign_sender.server_id.to_string();
  
  let server_repo = Arc::new(ServerRepoImpl);
  let server_service = servers_services::ServerService::new(server_repo);

  let server = server_service.get_server_by_id(server_id.as_str()).await?;


  match server.server_type {
    ServerTypeEnum::AWS => {
        let result = send_campaign_email_aws(campaign_id).await?;
        return Ok(AddMailToQueueResponse {
            status: StatusCode::OK.into(),
            message: "Campaign email sent successfully".to_string(),
        });
    }, 
    ServerTypeEnum::SMTP => {
        // let result = send_campaign_email_smtp(campaign_id, Uuid::parse_str(&server_id).unwrap()).await?;
        let result = enqueue_email(campaign_id, server.id).await?;
        return Ok(AddMailToQueueResponse {
            status: StatusCode::OK.into(),
            message: "Campaign email sent successfully".to_string(),
        });
    } 
  }

    // instead of sending the email, lets first enqueue the emails which will be sent by background process in interval...
}

pub async fn send_campaign_email_aws(
    campaign_id: Uuid,
) -> Result<CampaignSendResponse, AppError> {

    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let configuration_name = env::var("AWS_SES_CONFIGURATION_SET_NAME").expect("AWS_SES_CONFIGURATION_SET_NAME must be set in .env file");

    let contacts = get_unique_contacts_from_campaign(campaign_id).await?;

    let template = get_template_by_id(campaign.template_id.clone()).await
    .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    // Handle the Option
    let sender_id_string = match campaign.campaign_senders {
        Some(uuid) => uuid.to_string(), // Convert Uuid to String
        None => return Err(AppError::NotFoundError(Some("Sender ID not found for campaign.".to_string()))), // Handle None case
    };

    let campaign_sender_response = get_campaign_sender_by_id(sender_id_string).await
    .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let sender_email = campaign_sender_response.from_email;
    let sender_name = campaign_sender_response.from_name;
    //These variables are temporary

    let server_uuid = campaign_sender_response.server_id;
    let server_id = server_uuid.to_string();

    let client = aws_service::create_aws_client_db(&server_id).await;
    let request = client.list_email_identities();

    // Send the request and await the response
    let result = request.send().await.map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    // Print the identities (email addresses or domains)
    if let Some(identities) = result.email_identities {
        println!("Verified Email Identities:");
        for identity in identities {
            println!("{:?}", identity.identity_name);
        }
    } else {
        println!("No identities found.");
    }

    //This current logic may need to be changed while implementing queue
    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact)
            .await
            .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

        let body = Body::builder()
            .html(Content::builder()
                .data(parsed_html.clone())
                .charset("UTF-8")
                .build().map_err(|err| AppError::InternalServerError(Some(err.to_string())))?
            )
            .build();

        let message = Message::builder()
            .subject(Content::builder()
                .data(format!("Hello {}", contact.first_name))
                .charset("UTF-8")
                .build().map_err(|err| AppError::InternalServerError(Some(err.to_string())))?
            )
            .body(body)
            .build();

        let result = client.send_email()
            .from_email_address(format!("{} <{}>", sender_name, sender_email.clone()))
            .destination(Destination::builder()
                .to_addresses(contact.email.clone())
                .build()
            )
            .content(EmailContent::builder()
                .simple(message)
                .build()
            )
            .configuration_set_name(&configuration_name)
            .send()
            .await?;

        let new_mail = CreateMailRequest {
            id: result.message_id().unwrap().to_string(),
            mail_message: parsed_html,
            email: vec![contact.email.clone()],
            template_id: Some(Uuid::parse_str(&template.id)?),
            campaign_id: Some(campaign_id),
            sent_at: chrono::Utc::now(),
            status: "queued".to_string(),
            server_id: Some(server_uuid),
        };

        mail_service::create_mail(new_mail).await?;
    }
    Ok(CampaignSendResponse {
        campaign_id: campaign_id.to_string(),
        total_recipients: contacts.len(),
        status: "draft".to_string(),
    })
}

pub async fn add_lists_to_campaign(campaign_id: Uuid, list_ids: Vec<Uuid>) -> Result<Vec<NewListInCampaign>, AppError> {
    let campaign_list_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_list_repository);
    
    let result = campaign_list_service.add_lists_to_campaign(campaign_id, list_ids)
        .await?;

    Ok(result)
}

pub async fn send_campaign_email_smtp(
    campaign_id: Uuid,
    server_id: Uuid,
) -> Result<CampaignSendResponse, AppError> {
    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let contacts = get_unique_contacts_from_campaign(campaign_id).await?;

    let template = get_template_by_id(campaign.template_id.clone()).await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let sender_id_string = match campaign.campaign_senders {
        Some(uuid) => uuid.to_string(),
        None => return Err(AppError::NotFoundError(Some("Sender ID not found for campaign.".to_string()))),
    };

    let campaign_sender_response = get_campaign_sender_by_id(sender_id_string).await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let sender_email = campaign_sender_response.from_email;
    let server_service = ServerService::new(Arc::new(ServerRepoImpl));

    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact).await.map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

        let result = server_service.send_mail_with_smtp(
            server_id,
            &sender_email,
            vec![contact.email.clone()],
            None,
            None,
            &format!("Hello {}", contact.first_name),
            &parsed_html,
        ).await;

        match result {
            Ok(_) => {
                let new_mail = CreateMailRequest {
                    id: Uuid::new_v4().to_string(),
                    mail_message: parsed_html,
                    email: vec![contact.email.clone()],
                    template_id: Some(Uuid::parse_str(&template.id)?),
                    campaign_id: Some(campaign_id),
                    sent_at: chrono::Utc::now(),
                    status: "queued".to_string(),
                    server_id: Some(server_id),
                };
                mail_service::create_mail(new_mail).await.map_err(|err| {
                    AppError::InternalServerError(Some(format!("Failed to create mail: {}", err)))
                })?;
            },
            Err(e) => {
                return Err(AppError::InternalServerError(Some(format!("SEND MAIL ERROR VIA SMTP: {:?}", e))));
            }
        }
    }

    Ok(CampaignSendResponse {
        campaign_id: campaign_id.to_string(),
        total_recipients: contacts.len(),
        status: "draft".to_string(),
    })
}

/// a function to send a single email to a contact with smtp server...
pub async fn send_single_email_smtp (
    mail_id: String,
    campaign_id: Uuid,
    server_id: Uuid,
    email: &str,
    message: String,
    subject: String,
) -> Result<CampaignSendResponse, AppError> {
    let server_service = ServerService::new(Arc::new(ServerRepoImpl));

    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let sender_id_string = match campaign.campaign_senders {
        Some(uuid) => uuid.to_string(),
        None => return Err(AppError::NotFoundError(Some("Sender ID not found for campaign.".to_string()))),
    };

    let campaign_sender_response = get_campaign_sender_by_id(sender_id_string).await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let sender_email = campaign_sender_response.from_email;

    let result = server_service.send_mail_with_smtp(
        server_id,
        &sender_email,
        vec![email.to_string()],
        None,
        None,
        &subject,
        &message,
    ).await.map_err(|err| AppError::InternalServerError(Some(format!("{:?}", err))))?;

    let mail_status = "submitted".to_string();

    update_mail_status(mail_id, mail_status.clone()).await.map_err(|err| {
        AppError::InternalServerError(Some(format!("Failed to update mail status: {}", err)))
    })?;

    Ok(CampaignSendResponse {
        campaign_id: campaign_id.to_string(),
        total_recipients: 1,
        status: mail_status,
    })
}

/// a function to enqueue email for sending...
pub async fn enqueue_email(
    campaign_id: Uuid,
    server_id: Uuid
) -> Result<(), AppError> {
    let campaign = get_campaign_by_id(campaign_id.clone())
    .await
    .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let contacts = get_unique_contacts_from_campaign(campaign_id).await?;

    let template = get_template_by_id(campaign.template_id.clone()).await
    .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact).await.map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

        let new_mail = CreateMailRequest {
            id: Uuid::new_v4().to_string(),
            mail_message: parsed_html,
            email: vec![contact.email.clone()],
            template_id: Some(Uuid::parse_str(&template.id)?),
            campaign_id: Some(campaign_id),
            sent_at: chrono::Utc::now(),
            status: "queued".to_string(),
            server_id: Some(server_id),
        };

        mail_service::create_mail(new_mail).await.map_err(|err| {
            AppError::InternalServerError(Some(format!("Failed to create mail: {}", err)))
        })?;
    }

    Ok(())
}