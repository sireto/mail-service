use backend::models::list::{self, CreateListRequest, CreateListResponse, DeleteListResponse, ListResponse, UpdateListRequest, UpdatedListResponse};
use serde_json::json;
use reqwest::Client;
use uuid::Uuid;

#[tokio::test]
async fn test_get_lists() {
    // Start a mock server
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    // Corrected payload with proper null handling
    let payload = json!([
        {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "namespace_id": "987e6543-e89b-12d3-a456-426614174000",
            "name": "List Template 1",
            "description": "This is a test description for list template 1",
            "created_at": "2023-01-01T00:00:00",
            "updated_at": "2023-01-10T00:00:00"
        },
        {
            "id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
            "namespace_id": "c47ac10b-58cc-4372-a567-0e02b2c3d479",
            "name": "List Template 2",
            "description": "This is a test description for list template 2",
            "created_at": "2023-02-01T00:00:00",
            "updated_at": "2023-02-10T00:00:00"
        }
    ]);

    // Print the payload to ensure it's formatted correctly
    println!("Payload: {}", payload.to_string());

    // Create a mock for the GET request to /api/list_templates
    let _mock = server.mock("GET", "/api/list_templates")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(payload.to_string()) // Convert to string before passing to `with_body`
        .create();

    // Send the request to the mock server
    let client = Client::new();
    let response = client
        .get(&format!("{}/api/list_templates", url)) // Use mock_server URL
        .send()
        .await
        .expect("Failed to send request");

    // Ensure that the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into Vec<ListResponse>
    let lists: Vec<ListResponse> = response.json().await.expect("Failed to parse response");

    // Assert the response contains the expected list template
    assert_eq!(lists[0].name, "List Template 1");
    assert_eq!(lists[0].description, Some("This is a test description for list template 1".to_string()));
    assert_eq!(lists[1].name, "List Template 2");

    // Optionally, check if the mock was called
    _mock.assert();
}

#[tokio::test]
async fn test_create_list() {
    // Start a mock server
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    // Mock request payload
    let request_payload = CreateListRequest {
        name: "New List".to_string(),
        namespace_id: Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap(),
        description: "This is a new list".to_string(),
    };

    // Mock response payload
    let response_payload = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "New List",
        "created_at": "2023-01-01T00:00:00Z"
    });

    // Create a mock for the POST request to /api/lists
    let _mock = server.mock("POST", "/api/lists")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(response_payload.to_string())
        .create();

    // Send the request to the mock server
    let client = Client::new();
    let response = client
        .post(&format!("{}/api/lists", url))
        .json(&request_payload)
        .send()
        .await
        .expect("Failed to send request");

    // Ensure that the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into CreateListResponse
    let create_response: CreateListResponse = response.json().await.expect("Failed to parse response");

    // Assert the response contains the expected data
    assert_eq!(create_response.name, "New List");
    assert_eq!(create_response.id, "123e4567-e89b-12d3-a456-426614174000");

    // Optionally, check if the mock was called
    _mock.assert();
}


#[tokio::test]
async fn test_update_list() {
    // Start a mock server
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    // Mock request payload
    let request_payload = UpdateListRequest {
        name: "Updated List Name".to_string(),
        description: Some("Updated description".to_string()),
    };

    // Mock response payload
    let response_payload = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "Updated List Name",
        "updated_at": "2023-01-01T00:00:00"
    });

    // Create a mock for the PUT request to /api/lists/{id}
    let _mock = server.mock("PUT", "/api/lists/123e4567-e89b-12d3-a456-426614174000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_payload.to_string())
        .create();

    // Send the request to the mock server
    let client = Client::new();
    let response = client
        .put(&format!("{}/api/lists/123e4567-e89b-12d3-a456-426614174000", url))
        .json(&request_payload)
        .send()
        .await
        .expect("Failed to send request");

    // Ensure that the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into UpdatedListResponse
    let update_response: UpdatedListResponse = response.json().await.expect("Failed to parse response");

    // Assert the response contains the expected data
    assert_eq!(update_response.name, "Updated List Name");
    assert_eq!(update_response.id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());

    // Optionally, check if the mock was called
    _mock.assert();
}


#[tokio::test]
async fn test_delete_list() {
    // Start a mock server
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    // Mock response payload
    let response_payload = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "Deleted List"
    });

    // Create a mock for the DELETE request to /api/lists/{id}
    let _mock = server.mock("DELETE", "/api/lists/123e4567-e89b-12d3-a456-426614174000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_payload.to_string())
        .create();

    // Send the request to the mock server
    let client = Client::new();
    let response = client
        .delete(&format!("{}/api/lists/123e4567-e89b-12d3-a456-426614174000", url))
        .send()
        .await
        .expect("Failed to send request");

    // Ensure that the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into DeleteListResponse
    let delete_response: DeleteListResponse = response.json().await.expect("Failed to parse response");

    // Assert the response contains the expected data
    assert_eq!(delete_response.name, "Deleted List");
    assert_eq!(delete_response.id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());

    // Optionally, check if the mock was called
    _mock.assert();
}

#[tokio::test]
async fn test_get_list_by_id() {
    // Start a mock server
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    // Mock response payload
    let response_payload = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "namespace_id": "987e6543-e89b-12d3-a456-426614174000",
        "name": "List Template 1",
        "description": "This is a test description for list template 1",
        "created_at": "2023-01-01T00:00:00",
        "updated_at": "2023-01-10T00:00:00"
    });

    // Create a mock for the GET request to /api/lists/{id}
    let _mock = server.mock("GET", "/api/lists/123e4567-e89b-12d3-a456-426614174000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_payload.to_string())
        .create();

    // Send the request to the mock server
    let client = Client::new();
    let response = client
        .get(&format!("{}/api/lists/123e4567-e89b-12d3-a456-426614174000", url))
        .send()
        .await
        .expect("Failed to send request");

    // Ensure that the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into ListResponse
    let list_response: ListResponse = response.json().await.expect("Failed to parse response");

    // Assert the response contains the expected data
    assert_eq!(list_response.name, "List Template 1");
    assert_eq!(list_response.id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());

    // Optionally, check if the mock was called
    _mock.assert();
}