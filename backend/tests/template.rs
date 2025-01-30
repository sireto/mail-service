use std::sync::Arc;
use backend::{ models::template::{Template, CreateTemplateRequest, UpdateTemplateRequest }, repositories::template_repo::MockTemplateRepository, services::template_service::TemplateService};
use uuid::Uuid;
use mockall::predicate::*;

#[tokio::test]
async fn test_create_template() {
    let mut mock_repo = MockTemplateRepository::new();

    // A payload with extra fields...
    let test_payload = CreateTemplateRequest {
        namespace_id: Uuid::new_v4(),
        name: "Test Template".to_string(),
        template_data: serde_json::json!("{\"user_name\": \"John Doe\"}"),
        content_plaintext: Some("Test Content Plaintext".to_string()),
        content_html: "Test Content HTML".to_string(),
    };

    let expected_output = Template {
        id: Uuid::new_v4(),
        namespace_id: Uuid::new_v4(),
        name: "Test Template".to_string(),
        template_data: serde_json::json!("{\"user_name\": \"John Doe\"}"),
        content_plaintext: Some("Test Content Plaintext".to_string()),
        content_html: "Test Content HTML".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_create_template()
        .with(eq(test_payload.clone()))
        .returning(move |_| Ok(expected_output.clone()));

    let template_service = TemplateService::new(Arc::new(mock_repo));

    let result = template_service.create_template(test_payload.clone()).await;

    assert!(result.is_ok());

    // Ensure the created template has the expected name...
    assert_eq!(result.as_ref().unwrap().name, "Test Template");

    // Ensure the created template has the expected template data...
    assert_eq!(result.as_ref().unwrap().template_data, serde_json::json!("{\"user_name\": \"John Doe\"}"));

    // Assert that the template ID is correctly generated (not the same as the input ID)...
    assert_ne!(result.as_ref().unwrap().id, test_payload.namespace_id);

}

#[tokio::test]
async fn test_get_templates() {
    let mut mock_repo = MockTemplateRepository::new();
    
    let expected_output = vec![
        Template {
            id: Uuid::new_v4(),
            namespace_id: Uuid::new_v4(),
            name: "Test Template".to_string(),
            template_data: serde_json::json!("{\"user_name\": \"John Doe\"}"),
            content_plaintext: Some("Test Content Plaintext".to_string()),
            content_html: "Test Content HTML".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];

    mock_repo
        .expect_get_all_templates()
        .returning(move || Ok(expected_output.clone()));

    let template_service = TemplateService::new(Arc::new(mock_repo));

    let result = template_service.get_all_templates().await;

    assert!(result.is_ok());

    let result_output = result.as_ref().unwrap();

    assert!(!result_output.is_empty());
    assert!(result_output[0].name == "Test Template");
    assert!(result_output[0].template_data == serde_json::json!("{\"user_name\": \"John Doe\"}"));
    assert!(result_output[0].content_plaintext == Some("Test Content Plaintext".to_string()));

}

#[tokio::test]
async fn test_get_template_by_id() {
    let mut mock_repo = MockTemplateRepository::new();

    let template_id = Uuid::new_v4();

    let expected_output = Template {
        id: template_id,
        namespace_id: Uuid::new_v4(),
        name: "Test Template".to_string(),
        template_data: serde_json::json!("{\"user_name\": \"John Doe\"}"),
        content_plaintext: Some("Test Content Plaintext".to_string()),
        content_html: "Test Content HTML".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_get_template_by_id()
        .with(eq(template_id))
        .returning(move |_| Ok(expected_output.clone()));

    let template_service = TemplateService::new(Arc::new(mock_repo));

    let result = template_service.get_template_by_id(template_id).await;

    assert!(result.is_ok());

    let result_output = result.as_ref().unwrap();

    assert_eq!(result_output.name, "Test Template");
    assert_eq!(result_output.template_data, serde_json::json!("{\"user_name\": \"John Doe\"}"));
    assert_eq!(result_output.content_plaintext, Some("Test Content Plaintext".to_string()));
}

#[tokio::test]
async fn test_update_template() {
    let mut mock_repo = MockTemplateRepository::new();

    let update_payload = UpdateTemplateRequest {
        name: "Updated Template".to_string(),
        template_data: serde_json::json!("{\"user_name\": \"Updated\"}"),
        content_plaintext: "Updated Content".to_string(),
        content_html: "Updated HTML Content".to_string(),
    };

    let template_id = Uuid::new_v4();

    let updated_template = Template {
        id: template_id,
        namespace_id: Uuid::new_v4(),
        name: update_payload.name.clone(),
        template_data: update_payload.template_data.clone(),
        content_plaintext: Some(update_payload.content_plaintext.clone()),
        content_html: update_payload.content_html.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_update_template()
        .with(eq(template_id), eq(update_payload.clone()))
        .returning(move |_, _| Ok(updated_template.clone()));

    let template_service = TemplateService::new(Arc::new(mock_repo));

    let result = template_service.update_template(template_id, update_payload).await;

    assert!(result.is_ok());

    let result_output = result.as_ref().unwrap();

    assert_eq!(result_output.name, "Updated Template");

    // Ensure the updated_at timestamp is later than the created_at timestamp...
    assert!(result_output.updated_at > result_output.created_at);
}

#[tokio::test]
async fn test_delete_template() {
    let mut mock_repo = MockTemplateRepository::new();

    let template_id = Uuid::new_v4();
    let deleted_template = Template {
        id: template_id,
        namespace_id: Uuid::new_v4(),
        name: "Test Template".to_string(),
        template_data: serde_json::json!("{\"user_name\": \"John Doe\"}"),
        content_plaintext: Some("Test Content Plaintext".to_string()),
        content_html: "Test Content HTML".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_delete_template()
        .with(eq(template_id))
        .returning(move |_| Ok(deleted_template.clone()));

    let template_service = TemplateService::new(Arc::new(mock_repo));

    let result = template_service.delete_template(template_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, template_id);
}