use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::Instant;
use uuid::Uuid;

use backend::repositories::mail_repository::MockMailRepository;
use backend::servers::{
    servers_model::{Server, ServerTypeEnum, TlsTypeEnum},
    servers_repo::MockServerRepo,
    servers_services::ServerService,
};
use backend::services::mail_service::MailService;
use backend::{
    models::mail::{Mail, MailWithDetails},
    services::mail_service::MailServiceTrait,
};

#[tokio::test]
async fn test_process_mails_rate_limiting() {
    // mock server with rate_limit of 1 mail/sec
    let server_id = Uuid::new_v4();
    let fake_server = Server {
        id: server_id,
        active: true,
        host: "smtp.test".into(),
        smtp_username: "user1".into(),
        smtp_password: "pass".into(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::SSLTLS,
        port: 587,
        server_type: ServerTypeEnum::SMTP,
        aws_credentials: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        default_from_email: "noreply@test".into(),
        rate_limit: 1,
    };
    let mut mock_srv_repo = MockServerRepo::new();
    mock_srv_repo
        .expect_get_all_servers()
        .returning(move || Ok(vec![fake_server.clone()]));
    let server_service = Arc::new(ServerService::new(Arc::new(mock_srv_repo)));

    // mock mail repo that returns exactly one queued mail each tick...
    let mail_id = "m1".to_string();
    let mut mock_mail_repo = MockMailRepository::new();
    // every call to get_queued_mails returns our single pending mail...
    mock_mail_repo
        .expect_get_mails_by_status()
        .returning(move |status_arg, _| {
            if status_arg == "queued" {
                Ok(vec![MailWithDetails {
                    id: mail_id.clone(),
                    mail_message: "hi".into(),
                    template_id: None,
                    campaign_id: Some(Uuid::new_v4()),
                    server_id: Some(server_id),
                    sent_at: Utc::now(),
                    status: "pending".into(),
                    open: None,
                    clicks: 0,
                    scheduled_at: Utc::now(),
                    attempts: 0,
                    last_error: None,
                    email: "abc@example.com".into(),
                    reason: None,
                    from_name: None,
                }])
            } else {
                Ok(vec![])
            }
        });

    // record the instants when update_mail_status is called...
    let times = Arc::new(Mutex::new(Vec::new()));
    let times_clone = times.clone();
    mock_mail_repo
        .expect_update_mail_status()
        .returning(move |id, new_status| {
            // record the instant...
            times_clone.lock().unwrap().push(Instant::now());
            // return some dummy Mail back to caller...
            Ok(Mail {
                id: id.clone(),
                mail_message: "".into(),
                contact_id: Uuid::new_v4(),
                template_id: None,
                campaign_id: None,
                sent_at: Utc::now(),
                status: new_status.to_string(),
                open: None,
                clicks: 0,
                server_id: None,
                scheduled_at: Utc::now(),
                attempts: 0,
                last_error: None,
            })
        });

    let mail_service = Arc::new(MailService::new(Arc::new(mock_mail_repo)));

    // run the real worker (process) in the background...
    let handle = tokio::spawn({
        let mail_service = mail_service.clone();
        let server_service = server_service.clone();
        async move {
            mail_service.process_mails(server_service).await.unwrap();
        }
    });
    tokio::time::sleep(Duration::from_secs(10)).await;
    handle.abort();

    // assert we got at least two updates, each ≥1s apart...
    let sent = times.lock().unwrap();
    assert!(sent.len() >= 2, "sent.len()={}", sent.len());
    assert!(
        sent.len() <= 10,
        "Expected at most 10 mails sent, but got {}",
        sent.len()
    );
    for window in sent.windows(2) {
        let delta = window[1] - window[0];
        assert!(delta >= Duration::from_secs(1), "two sends too close: {:?}", delta);
    }
}
