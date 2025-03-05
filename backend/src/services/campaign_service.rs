use crate::{models::{campaign::{CampaignSendResponse, DeleteCampaignResponse, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse}, campaign_lists::NewListInCampaign, mail::CreateMailRequest}, repositories::{campaign::{self, CampaginRepositoryImpl, CampaignRepository}, campaign_lists_repo::{CampaignListRepository, CampaignListRepositoryImpl}, list_contact_repo::ListContactRepositoryImpl, mail_repository::MailRepositoryImpl},servers::{servers_repo::ServerRepoImpl, servers_services::{ServerService, ServerServiceTrait}}, utils::contact_lists_functions::populate_contact_template};
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
use aws_sdk_sesv2::types::{Body, Content, Destination, Message, EmailContent};
use crate::services::{aws_service, list_service::ListContactService, template_service::get_template_by_id};
use anyhow::{anyhow, Result};

use super::{campaign_sender_service::get_campaign_sender_by_id, mail_service::MailService };
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

    async fn get_lists_from_campaign(&self, campaign_id: Uuid) -> Result<Vec<ListResponse>, diesel::result::Error> {
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



pub async fn create_campaign( payload: ExtendedCreateCampaignRequest) -> Result<CreateCampaignResponse, (StatusCode, String)> {
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

    let response = campaign_service.create_campaign(new_campaign).await;

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);

    let response = match response {
        Ok(campaign) => {
            let list_ids: Vec<Uuid> = payload
            .list_ids
            .iter()
            .map(|id| Uuid::parse_str(id).expect("Invalid UUID format"))
            .collect();

            let response = campaign_list_service.add_lists_to_campaign(campaign.id, list_ids).await;

            match response {
                Ok(lists) => {
                    println!("Lists successfully added to the campaign: {:?}", lists);
                },
                Err(err) => {
                    println!("Failed to add lists to the campaign: {:?}", err);
                }
            }

            CreateCampaignResponse {
                id: campaign.id, 
                campaign_name: campaign.campaign_name, 
                template_id: campaign.template_id, 
                namespace_id: campaign.namespace_id, 
                status: campaign.status, 
                campaign_senders: campaign.campaign_senders, 
                scheduled_at: campaign.scheduled_at, 
                created_at: campaign.created_at, 
                updated_at: campaign.updated_at
            }
        }, 
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

pub async fn get_all_campaigns() -> Result<Vec<GetCampaignResponse>, (StatusCode, String)> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let all_campaigns = campaign_service.get_all_campaigns().await;

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    
    let response = match all_campaigns {
        Ok(campaigns) => {
            let mut responses = Vec::new();
            for campaign in campaigns {
                let lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
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
            responses
        },
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
    
    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    let lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|err|(StatusCode::NOT_FOUND, err.to_string()))?;

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

pub async fn update_campaign(campaign_id: String, payload: UpdateCampaignRequest) -> Result<UpdateCampaignResponse, (StatusCode, String)> {
    
    let uuid_id = Uuid::parse_str(&campaign_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid campaign ID format".to_string()))?;
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    
    let updated_campaign_response = campaign_service.update_campaign(uuid_id, payload.clone()).await;

    let response_campaign = match updated_campaign_response {
        Ok(campaign) => {
            // get all the existing lists from the table...
            let existing_lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|err|(StatusCode::NOT_FOUND, err.to_string()))?;

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

            // Delete lists that are no longer linked to the campaign
            if !to_delete.is_empty() {
            campaign_list_service
                .delete_lists_from_campaign(campaign.id, to_delete.clone())
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            }

            // Add new lists to the campaign
            if !to_add.is_empty() {
                campaign_list_service
                    .add_lists_to_campaign(campaign.id, to_add.clone())
                    .await
                    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            }

            let lists = campaign_list_service.get_lists_from_campaign(campaign.id).await.map_err(|err|(StatusCode::NOT_FOUND, err.to_string()))?;


            // separate the above lists into two groups where the lists from the arguments are present in the list and the lists from the arguments are not present in the list...


            let response = UpdateCampaignResponse {
                id: campaign.id, 
                campaign_name: campaign.campaign_name, 
                template_id:  campaign.template_id, 
                namespace_id: campaign.namespace_id, 
                status: campaign.status, 
                campaign_senders: campaign.campaign_senders, 
                scheduled_at: Some(campaign.scheduled_at), 
                updated_at: Some(campaign.updated_at),
                lists
            };

            response
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
) -> Result<CampaignSendResponse, anyhow::Error> {
    let client = aws_service::create_aws_client().await;
    let campaign_uuid = Uuid::parse_str(&campaign_id)?;
    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch campaign ({}): {}", status_code, message)
        })?;

    let configuration_name = env::var("AWS_SES_CONFIGURATION_SET_NAME").expect("AWS_SES_CONFIGURATION_SET_NAME must be set in .env file");

    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    let list_ids: Vec<Uuid> = campaign_list_service.get_lists_from_campaign(campaign_uuid).await?
    .into_iter()
    .map(|list| list.id)
    .collect();

    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let list_contact_service = ListContactService::new(list_contact_repository);
    let contacts = list_contact_service.get_contacts_from_lists(list_ids.clone()).await?;


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

    //This current logic may need to be changed while implementing queue
    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact).await?;

        let body = Body::builder()
            .html(Content::builder()
                .data(parsed_html.clone())
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
            .configuration_set_name(&configuration_name)
            .send()
            .await;

        match result {
            Ok(response) => {
                // after successfull send mail, at it to the mail table with the pending status...
                let new_mail = CreateMailRequest {
                    id: response.message_id().unwrap().to_string(),
                    mail_message: parsed_html,
                    email: vec![contact.email.clone()],
                    template_id: Some(Uuid::parse_str(&template.id)?),
                    campaign_id: Some(campaign_uuid),
                    sent_at: chrono::Utc::now(),
                    status: "pending".to_string(),
                };

                mail_service::create_mail(new_mail).await.map_err(|(status_code, message)| {
                    anyhow!("Failed to create mail ({}): {}", status_code, message)
                })?;

            },
            Err(e) => {
                //Handle the send mail Error from the SES side only ( not bounces )...
                println!("SEND MAIL ERROR FROM SES: {:?}", e);
            }
        }
    }
    Ok(CampaignSendResponse {
        campaign_id,
        total_recipients: contacts.len(),
        status: "draft".to_string(),
    })
}


pub async fn add_lists_to_campaign(campaign_id: Uuid, list_ids: Vec<Uuid>) -> Result<Vec<NewListInCampaign>, (StatusCode, String)> {
    let campaign_list_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_list_repository);
    
    campaign_list_service.add_lists_to_campaign(campaign_id, list_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn send_campaign_email_smtp(
    campaign_id: String,
    server_id: Uuid,
) -> Result<CampaignSendResponse, anyhow::Error> {
    let campaign = get_campaign_by_id(campaign_id.clone())
        .await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch campaign ({}): {}", status_code, message)
        })?;

    let campaign_id = Uuid::parse_str(&campaign_id)?;
    let campaign_lists_repository = Arc::new(CampaignListRepositoryImpl);
    let campaign_list_service = CampaignListService::new(campaign_lists_repository);
    let list_ids: Vec<Uuid> = campaign_list_service.get_lists_from_campaign(campaign_id).await?
    .into_iter()
    .map(|list| list.id)
    .collect();

    let list_contact_repository = Arc::new(ListContactRepositoryImpl);
    let list_contact_service = ListContactService::new(list_contact_repository);
    let contacts = list_contact_service.get_contacts_from_lists(list_ids).await?;

    let template = get_template_by_id(campaign.template_id.clone()).await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch template ({}): {}", status_code, message)
        })?;

    let sender_id_string = match campaign.campaign_senders {
        Some(uuid) => uuid.to_string(),
        None => return Err(anyhow!("Sender ID not found for campaign.")),
    };

    let campaign_sender_response = get_campaign_sender_by_id(sender_id_string).await
        .map_err(|(status_code, message)| {
            anyhow!("Failed to fetch sender ({}): {}", status_code, message)
        })?;

    let sender_email = campaign_sender_response.from_email;
    let server_service = ServerService::new(Arc::new(ServerRepoImpl));

    for contact in contacts.clone() {
        let parsed_html = populate_contact_template(&template, &contact).await?;

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
                    campaign_id: Some((campaign_id)),
                    sent_at: chrono::Utc::now(),
                    status: "pending".to_string(),
                };
                mail_service::create_mail(new_mail).await.map_err(|(status_code, message)| {
                    anyhow!("Failed to create mail ({}): {}", status_code, message)
                })?;
            },
            Err(e) => {
                println!("SEND MAIL ERROR VIA SMTP: {:?}", e);
            }
        }
    }

    Ok(CampaignSendResponse {
        campaign_id: campaign_id.to_string(),
        total_recipients: contacts.len(),
        status: "draft".to_string(),
    })
}
