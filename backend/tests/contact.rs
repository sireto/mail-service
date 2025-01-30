use std::sync::Arc;
use backend::{models::contact::{Contact, CreateContactRequest, UpdateContactRequest }, repositories::contact::MockContactRepository, services::contact::ContactService};
use uuid::Uuid;
use mockall::predicate::*;

#[tokio::test]
async fn test_create_contact() {
    let mut mock_repo = MockContactRepository::new();

    // Creating a payload with extra fields
    let test_payload = CreateContactRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@gmail.com".to_string(),
        attribute: None,
    };

    // Define the expected output without the extra field
    let expected_output = Contact {
        id: Uuid::new_v4(),
        first_name: "Paul".to_string(),
        last_name: "Jack".to_string(),
        email: "paul@gmail.com".to_string(),
        attribute: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_create_contact()
        .with(eq(test_payload.clone()))
        .returning(move |_| Ok(expected_output.clone()));

    let contact_service = ContactService::new(Arc::new(mock_repo));

    let result = contact_service.create_contact(test_payload.clone()).await;

    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap().email, "paul@gmail.com");
    assert_eq!(result.unwrap().attribute, None);
}

#[tokio::test]
async fn test_get_all_contacts() {
    let mut mock_repo = MockContactRepository::new();

    let expected_output = vec![
        Contact {
            id: Uuid::new_v4(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@gmail.com".to_string(),
            attribute: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];

    mock_repo
        .expect_get_all_contacts()
        .returning(move || Ok(expected_output.clone()));

    let contact_service = ContactService::new(Arc::new(mock_repo));

    let result = contact_service.get_all_contacts().await;

    assert!(result.is_ok());
    assert!(!result.as_ref().unwrap().is_empty());
    assert!(result.as_ref().unwrap().len() == 1);
    assert_eq!(result.as_ref().unwrap()[0].email, "john@gmail.com");
}

#[tokio::test]
async fn test_update_contact() {
    let mut mock_repo = MockContactRepository::new();

    let test_id = Uuid::new_v4();

    let test_payload = UpdateContactRequest {
        first_name: "Hank".to_string(),
        last_name: "Doe".to_string(),
        email: "hank@gmail.com".to_string(),
        attribute: Some(serde_json::json!("\"address\": \"Shinjuku\"")),
    };

    let expected_output = Contact {
        id: test_id,
        first_name: "Jank".to_string(),
        last_name: "Doe".to_string(),
        email: "hank@gmail.com".to_string(),
        attribute: Some(serde_json::json!("\"address\": \"Shinjuku\"")),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_update_contact()
        .with(eq(test_id), eq(test_payload.clone()))
        .returning(move |_, _| Ok(expected_output.clone()));

    let contact_service = ContactService::new(Arc::new(mock_repo));

    let result = contact_service.update_contact(test_id, test_payload.clone()).await;

    assert!(result.is_ok());

    let updated_result = result.unwrap();

    assert_eq!(updated_result.first_name, "Jank");

    // Ensure the last name remains unchanged...
    assert_eq!(updated_result.last_name, "Doe");

    // Ensure the email remains the same...
    assert_eq!(updated_result.email, "hank@gmail.com");

    // Ensure the attribute contains the expected JSON value...
    assert_eq!(
        updated_result.attribute,
        Some(serde_json::json!("\"address\": \"Shinjuku\""))
    );

    // Ensure timestamps exist
    assert!(updated_result.created_at <= chrono::Utc::now());
    assert!(updated_result.updated_at <= chrono::Utc::now());
}

#[tokio::test]
async fn test_delete_contact() {
    let mut mock_repo = MockContactRepository::new();

    let test_id = Uuid::new_v4();

    let expected_output = Contact {
        id: test_id,
        first_name: "Jank".to_string(),
        last_name: "Doe".to_string(),
        email: "hank@gmail.com".to_string(),
        attribute: Some(serde_json::json!("\"address\": \"Shinjuku\"")),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Mocking the expected behavior of deleting a contact..
    mock_repo
        .expect_delete_contact()
        .with(eq(test_id))
        .returning(move |_| Ok(expected_output.clone()));

    let contact_service = ContactService::new(Arc::new(mock_repo));

    // Call the service method to delete a contact...
    let result = contact_service.delete_contact(test_id).await;

    assert!(result.is_ok()); // Ensures the result is Ok...
}

#[tokio::test]
async fn test_get_contact_by_id() {
    let mut mock_repo = MockContactRepository::new();

    let test_id = Uuid::new_v4();

    let expected_output = Contact {
        id: test_id,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@gmail.com".to_string(),
        attribute: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Mocking the expected behavior of getting a contact by id...
    mock_repo
        .expect_get_contact_by_id()
        .with(eq(test_id))
        .returning(move |_| Ok(expected_output.clone()));

    let contact_service = ContactService::new(Arc::new(mock_repo));

    // Call the service method to get the contact by id...
    let result = contact_service.get_contact_by_id(test_id).await;

    assert!(result.is_ok()); // Ensures the result is Ok...
    let contact = result.unwrap();

    assert_eq!(contact.id, test_id);
    assert_eq!(contact.first_name, "John");
    assert_eq!(contact.last_name, "Doe");
    assert_eq!(contact.email, "john@gmail.com");
    assert_eq!(contact.attribute, None);
    assert!(contact.created_at <= chrono::Utc::now());
    assert!(contact.updated_at <= chrono::Utc::now());
}