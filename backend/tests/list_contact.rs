use std::sync::Arc;
use backend::models::list_contacts::NewContactInList;
use backend::repositories::list_contact_repo::MockListContactRepository;
use backend::services::list_service::ListContactService;
use uuid::Uuid;
use mockall::predicate::*;



#[tokio::test]
async fn test_add_contacts_to_list() {
    // Create the mock repository
    let mut mock_repo = MockListContactRepository::new();

    // Define test input data (list_id and contact_ids)
    let list_id = Uuid::new_v4();
    let contact_ids = vec![Uuid::new_v4(), Uuid::new_v4()];

    // Define the expected output data (NewContactInList)
    let expected_output = vec![
        NewContactInList {
            list_id: list_id.clone(),
            contact_id: contact_ids[0].clone(),
        },
        NewContactInList {
            list_id: list_id.clone(),
            contact_id: contact_ids[1].clone(),
        },
    ];

    // Define mock behavior for `add_contacts_to_list`
    mock_repo
        .expect_add_contacts_to_list()
        .with(eq(list_id), eq(contact_ids.clone()))
        .returning(move |_, _| Ok(expected_output.clone()));

    // Create the service using the mock repository
    let contact_service = ListContactService::new(Arc::new(mock_repo));

    // Call the method
    let result = contact_service.add_contacts_to_list(list_id, contact_ids.clone()).await;

    // Assert that the result is Ok and the returned data matches expected output
    assert!(result.is_ok());
    let added_contacts = result.unwrap();
    assert_eq!(added_contacts.len(), 2);
    assert_eq!(added_contacts[0].list_id, list_id);
    assert_eq!(added_contacts[0].contact_id, contact_ids[0]);
    assert_eq!(added_contacts[1].list_id, list_id);
    assert_eq!(added_contacts[1].contact_id, contact_ids[1]);
}



#[tokio::test]
async fn test_delete_contacts_from_list() {
    // Directly create a mock repository (without Mutex)
    let mut mock_repo = MockListContactRepository::new();

    // Set up test data
    let list_id = Uuid::new_v4();
    let contact_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

    // Define the expected result for the mock method
    let expected_deleted_count = 3;

    // Set up the mock expectation
    mock_repo
        .expect_delete_contacts_from_list()
        .with(eq(list_id), eq(contact_ids.clone()))  // Match the arguments
        .returning(move |_, _| Ok(expected_deleted_count)); // Return the expected count

    // Create the service using the mock repo
    let contact_service = ListContactService::new(Arc::new(mock_repo));

    // Call the method under test
    let result = contact_service.delete_contacts_from_list(list_id, contact_ids.clone()).await;

    // Assertions
    assert!(result.is_ok()); // Ensure it returned Ok
    let deleted_count = result.unwrap();
    assert_eq!(deleted_count, expected_deleted_count); // Verify the number of deleted contacts
}


