use std::sync::Arc;

use backend::{models::list::{CreateListRequest, List, UpdateListRequest,}, repositories::list_repo::MockListRepository, services::list_service::ListService};

use uuid::Uuid;


#[tokio::test]
async fn test_get_list_by_id1() {
    let mut mock_repo = MockListRepository::new();
    let namespace_id = Uuid::new_v4();
    let list_id = Uuid::new_v4();

    let list = List {
        id: list_id,
        name: "Sample List".to_string(),
        namespace_id,
        description: Some("Description".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo.expect_get_list_by_id()
        .returning(move |ns_id, l_id| {
            assert_eq!(ns_id, namespace_id);
            assert_eq!(l_id, list_id);
            Ok(list.clone())
        });

    let service = ListService::new(Arc::new(mock_repo));

    let result = service.get_list_by_id(namespace_id, list_id).await;

    assert!(result.is_ok());
    let response_list = result.unwrap();
    assert_eq!(response_list.id, list_id);
    assert_eq!(response_list.name, "Sample List");
    assert_eq!(response_list.description.unwrap(), "Description");
}



#[tokio::test]
async fn test_create_list() {
    let mut mock_repo = MockListRepository::new();

    let request = CreateListRequest {
        name: "Request List".to_string(),
        namespace_id: Uuid::new_v4(),
        description: "Request Description".to_string(),
    };

    // Mock behavior: use request data to create a list in DB
    mock_repo
        .expect_create_list()
        .returning(|req| Ok(List {
            id: Uuid::new_v4(),
            name: req.name.clone(),
            namespace_id: req.namespace_id,
            description: Some(req.description.clone()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }));

    let service = ListService::new(Arc::new(mock_repo));

    let result = service.create_list(request).await;

    assert!(result.is_ok());
    let created = result.unwrap();

    assert_eq!(created.name, "Request List");
}


#[tokio::test]
async fn test_update_list() {
    let mut mock_repo = MockListRepository::new();
    let namespace_id = Uuid::new_v4();
    let list_id = Uuid::new_v4();

    let updated_list = List {
        id: list_id,
        name: "Updated List".to_string(),
        namespace_id,
        description: Some("Updated Description".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let update_request = UpdateListRequest {
        name: "Updated List".to_string(),
        description: Some("Updated Description".to_string()),
    };

    mock_repo.expect_update_list()
        .returning(move |ns_id, l_id, _payload| {
            assert_eq!(ns_id, namespace_id);
            assert_eq!(l_id, list_id);
            Ok(updated_list.clone())
        });

    let service = ListService::new(Arc::new(mock_repo));

    let result = service.update_list(namespace_id, list_id, update_request).await;

    assert!(result.is_ok());
    let response_list = result.unwrap();
    assert_eq!(response_list.id, list_id);
    assert_eq!(response_list.name, "Updated List");
}


#[tokio::test]
async fn test_delete_list() {
    let mut mock_repo = MockListRepository::new();
    let namespace_id = Uuid::new_v4();
    let list_id = Uuid::new_v4();

    let list = List {
        id: list_id,
        name: "Sample List".to_string(),
        namespace_id,
        description: Some("Description".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    mock_repo.expect_delete_list()
        .returning(move |_, _| Ok(list.clone()));

    let service = ListService::new(Arc::new(mock_repo));

    let result = service.delete_list(namespace_id, list_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, list_id);
}
