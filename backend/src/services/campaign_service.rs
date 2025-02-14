use crate::{models::campaign::{CampaignSendResponse, DeleteCampaignResponse, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse}, repositories::{campaign::{self, CampaginRepositoryImpl, CampaignRepository}, list_contact_repo::ListContactRepositoryImpl}, utils::contact_lists_functions::populate_contact_template};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::campaign::{
    Campaign, 
    CreateCampaignRequest, 
    CreateCampaignResponse,
};
use aws_sdk_sesv2::types::{Body, Content, Destination, Message, EmailContent};
use crate::{services::{aws_service, list_service::ListContactService, template_service::get_template_by_id}};
use anyhow::{anyhow, Result};

use super::campaign_sender_service::get_campaign_sender_by_id;


pub struct CampaignService {
    repository: Arc<dyn CampaignRepository + Send +Sync>
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



pub async fn create_campaign( payload: CreateCampaignRequest) -> Result<CreateCampaignResponse, (StatusCode, String)> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let response = campaign_service.create_campaign(payload).await;

    let response = match response {
        Ok(campaign) => CreateCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            template_id: campaign.template_id, 
            namespace_id: campaign.namespace_id, 
            status: campaign.status, 
            campaign_senders: campaign.campaign_senders, 
            scheduled_at: campaign.scheduled_at, 
            created_at: campaign.created_at, 
            updated_at: campaign.updated_at
        }, 
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

pub async fn get_all_campaigns() -> Result<Vec<GetCampaignResponse>, (StatusCode, String)> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let all_campaigns = campaign_service.get_all_campaigns().await;

    let response = match all_campaigns {
        Ok(campaigns) => campaigns.into_iter().map(|campaign| GetCampaignResponse {
            id: campaign.id,
            campaign_name: campaign.campaign_name,
            template_id: campaign.template_id,
            namespace_id: campaign.namespace_id,
            status: campaign.status,
            campaign_senders: campaign.campaign_senders,
            scheduled_at: campaign.scheduled_at,
            created_at: campaign.created_at,
            updated_at: campaign.updated_at,
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string())),
    };

    Ok(response)
}

pub async fn get_campaign_by_id(campaign_id: String) -> Result<GetCampaignResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&campaign_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid campaign ID format".to_string()))?;

    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let campaign = campaign_service.get_campaign_by_id(uuid_id).await;

    let campaign = campaign.map_err(|err|(StatusCode::NOT_FOUND, err.to_string()))?;

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
    };

    Ok(campaign_response)
}

pub async fn update_campaign(campaign_id: String, payload: UpdateCampaignRequest) -> Result<UpdateCampaignResponse, (StatusCode, String)> {
    
    let uuid_id = Uuid::parse_str(&campaign_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid campaign ID format".to_string()))?;
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    
    let updated_campaign_response = campaign_service.update_campaign(uuid_id, payload).await;

    let response_campaign = match updated_campaign_response {
        Ok(campaign) => UpdateCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            template_id:  campaign.template_id, 
            namespace_id: campaign.namespace_id, 
            status: campaign.status, 
            campaign_senders: campaign.campaign_senders, 
            scheduled_at: Some(campaign.scheduled_at), 
            updated_at: Some(campaign.updated_at)
        }, 
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response_campaign)
}

pub async fn delete_campaign(campaign_id: String)->Result<DeleteCampaignResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&campaign_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let deleted_campaign_response = campaign_service.delete_campaign(uuid_id).await;

    let deleted_campaign = match deleted_campaign_response {
        Ok(campaign) => DeleteCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            status: campaign.status
        }, 
        Err(err)=> return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(deleted_campaign)
}

pub async fn send_campaign_email(
    campaign_id: String,
    list_id: String,
) -> Result<CampaignSendResponse, anyhow::Error> {
    let client = aws_service::create_aws_client().await;
    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch campaign ({}): {}", status_code, message)
        })?;

    // This will come from lists table, but for now it is hardcoded
    // let lists = vec![Uuid::parse_str("2af1e68e-0d28-455b-b88a-8cb1c2a70a01")?];
    let lists=  vec![Uuid::parse_str(&list_id)?];

    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let list_contact_service = ListContactService::new(list_contact_repository);
    let contacts = list_contact_service.get_contacts_from_lists(lists).await?;

    let template = get_template_by_id(campaign.template_id.clone()).await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch template ({}): {}", status_code, message)
        })?;

    // Handle the Option
    let sender_id_string = match campaign.campaign_senders {
        Some(uuid) => uuid.to_string(), // Convert Uuid to String
        None => return Err(anyhow!("Sender ID not found for campaign.")), // Handle None case
    };

    let campaign_sender_response = get_campaign_sender_by_id(sender_id_string).await
    .map_err(|(status_code, message)| {
        anyhow!("Failed to fetch sender ({}): {}", status_code, message)
    })?;

    let sender_email = campaign_sender_response.from_email;
    //These variables are temporary
    let mut success_mails = Vec::new();
    let mut failed_emails = Vec::new();
    

    //This current logic may need to be changed while implementing queue
    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact).await?;

        let body = Body::builder()
            .html(Content::builder()
                .data(parsed_html)
                .charset("UTF-8")
                .build()?
            )
            .build();

        let message = Message::builder()
            .subject(Content::builder()
                .data(format!("Hello {}", contact.first_name))
                .charset("UTF-8")
                .build()?
            )
            .body(body)
            .build();

        let result = client.send_email()
            .from_email_address(sender_email.clone())
            .destination(Destination::builder()
                .to_addresses(contact.email.clone())
                .build()
            )
            .content(EmailContent::builder()
                .simple(message)
                .build()
            )
            .send()
            .await;

        match result {
            Ok(response) => {
                //this will actually be stored in the mail table in the db later
                success_mails.push(response.message_id().unwrap().to_string());
                println!("{:?}",success_mails);
            },
            Err(e) => {
                //The failed emails will be stored in bounce_logs table later
                failed_emails.push(contact.email.clone());
                anyhow!("SES SDK Error: {}", e);
            }
        }
    }
    Ok(CampaignSendResponse {
        campaign_id,
        total_recipients: contacts.len(),
        status: "draft".to_string(),
    })
}