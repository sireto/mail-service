use std::sync::Arc;
use axum::http::StatusCode;
use backend::servers::servers_model::{Server, ServerRequest, TlsTypeEnum};
use backend::servers::servers_repo::MockServerRepo;
use backend::servers::servers_services::{ServerService, ServerServiceTrait};
use chrono::Utc;
use uuid::Uuid;
use mockall::predicate::*;

#[tokio::test]
async fn test_create_server() {
    let mut mock_repo = MockServerRepo::new();
    let test_payload = ServerRequest {
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: "securePassword123!".to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
    };
    let expected_server = Server {
        id: Uuid::new_v4(),
        active: true,
        host: test_payload.host.clone(),
        smtp_username: test_payload.smtp_username.clone(),
        smtp_password: test_payload.smtp_password.clone(),
        namespace_id: test_payload.namespace_id,
        tls_type: test_payload.tls_type.clone(),
        port: test_payload.port,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    mock_repo
        .expect_create_server()
        .with(eq(test_payload.clone()))
        .returning(move |_| Ok(expected_server.clone()));
    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.create_server(test_payload).await;
    assert!(result.is_ok());
    let server = result.unwrap();
    assert_eq!(server.host, "smtp.example.com");
    assert_eq!(server.smtp_username, "user@example.com");
}

#[tokio::test]
async fn test_get_all_servers() {
    let mut mock_repo = MockServerRepo::new();
    let expected_servers = vec![
        Server {
            id: Uuid::new_v4(),
            active: true,
            host: "smtp.example.com".to_string(),
            smtp_username: "user@example.com".to_string(),
            smtp_password: "securePassword123!".to_string(),
            namespace_id: Uuid::new_v4(),
            tls_type: TlsTypeEnum::STARTTLS,
            port: 587,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];
    mock_repo.expect_get_all_servers().returning(move || Ok(expected_servers.clone()));
    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.get_all_servers().await;

    let result_values = result.clone().unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
    assert_eq!(result_values[0].host, "smtp.example.com");
}

#[tokio::test]
async fn test_get_server_by_id() {
    let mut mock_repo = MockServerRepo::new();
    let server_id = Uuid::new_v4();
    let expected_server = Server {
        id: server_id,
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: "securePassword123!".to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    mock_repo.expect_get_server_by_id().with(eq(server_id)).returning(move |_| Ok(expected_server.clone()));
    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.get_server_by_id(&server_id.to_string()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, server_id);
}

#[tokio::test]
async fn test_get_server_by_id_not_found() {
    let mut mock_repo = MockServerRepo::new();
    let server_id = Uuid::new_v4();
    
    use diesel::result::Error as DieselError;
    mock_repo.expect_get_server_by_id()
        .with(eq(server_id))
        .returning(move |_| Err(DieselError::NotFound));

    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.get_server_by_id(&server_id.to_string()).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::NOT_FOUND);
}



#[tokio::test]
async fn test_update_server() {
    let mut mock_repo = MockServerRepo::new();
    let server_id = Uuid::new_v4();
    let update_payload = ServerRequest {
        active: true,
        host: "smtp.updated.com".to_string(),
        smtp_username: "updated@example.com".to_string(),
        smtp_password: "newSecurePassword!".to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::NONE,
        port: 465,
    };
    let updated_server = Server {
        id: server_id,
        active: true,
        host: update_payload.host.clone(),
        smtp_username: update_payload.smtp_username.clone(),
        smtp_password: update_payload.smtp_password.clone(),
        namespace_id: update_payload.namespace_id,
        tls_type: update_payload.tls_type.clone(),
        port: update_payload.port,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    mock_repo.expect_update_server().with(eq(server_id), eq(update_payload.clone())).returning(move |_, _| Ok(updated_server.clone()));
    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.update_server(&server_id.to_string(), update_payload).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().host, "smtp.updated.com");
}

#[tokio::test]
async fn test_delete_server() {
    let mut mock_repo = MockServerRepo::new();
    let server_id = Uuid::new_v4();
    let expected_server = Server {
        id: server_id,
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: "securePassword123!".to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    mock_repo.expect_delete_server().with(eq(server_id)).returning(move |_| Ok(expected_server.clone()));
    let server_service = ServerService::new(Arc::new(mock_repo));
    let result = server_service.delete_server(&server_id.to_string()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, server_id);
}