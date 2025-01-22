// use serde::{ Serialize, Deserialize };
// use uuid::Uuid;
// use std::collections::HashMap;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Template {
//     pub id: Uuid,
//     pub name: String,
//     pub namespace_id: Uuid,
//     pub template_data: HashMap<String, String>,
//     pub content_plaintext: String,
//     pub content_html: String,
//     pub created_at: String,
//     pub updated_at: String,
// }

// #[tokio::test]
// async fn get_template_handler() {
//     let url = "http://localhost:8000/api/templates";

//     let response = reqwest::Client::new()
//         .get(url)
//         .send()
//         .await
//         .expect("Failed to send request");

//     println!("Response status: {}", response.status());
    
//     // First assert the status code is successful
//     assert!(response.status().is_success());

//     // Parse the response body as a vector of templates
//     let templates: Vec<Template> = response.json().await.expect("Failed to parse response body");
    
//     // Assert that we got at least one template
//     assert!(!templates.is_empty());
    
//     // Get the first template for detailed assertions
//     let template = &templates[0];
//     println!("{}",template.name);
    
//     // Assert specific values
//     assert_eq!(template.id.to_string(), "abb71ec2-660a-4e63-89b2-5a78fc48bdda");
//     assert_eq!(template.name, "Welcome Email");
//     assert_eq!(template.namespace_id.to_string(), "8aba9d41-8e8f-424c-84b8-4933d54ede5e");
//     assert_eq!(template.content_plaintext, "Hello, welcome to our service!");
//     assert_eq!(template.content_html, "<h1>Hello</h1><p>Welcome to our service!</p>");
    
//     // Assert template_data contains expected key-value pairs
//     assert_eq!(template.template_data.get("key1"), Some(&"value1".to_string()));
//     assert_eq!(template.template_data.get("key2"), Some(&"value2".to_string()));

    
//     // Assert timestamps are present and in the expected format
//     assert!(template.created_at.starts_with("2025-01-20T"));
//     assert!(template.updated_at.starts_with("2025-01-20T"));
// }


// use backend::route;
// use axum::{
//     body::Body,
//     extract::connect_info::MockConnectInfo,
//     http::{self, Request, StatusCode},
// };
// use http_body_util::BodyExt; // for `collect`
// use serde_json::{json, Value};
// use tokio::net::TcpListener;
// use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

// use backend::db;
// use backend::appState::AppState;
// use std::{net::SocketAddr, sync::Arc};


// #[tokio::test]
// async fn get_template_test() {

//     let pool = db::establish_connection();
//     let app_state = Arc::new(AppState::new(pool));
//     let app = route::create_router(app_state);

//     let response = app
//         .oneshot(
//             Request::builder()
//             .uri("/api/templates")
//             .body(Body::empty()).unwrap())
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = response.into_body().collect().await.unwrap().to_bytes();
//         let body: Value = serde_json::from_slice(&body).unwrap();
//         assert_eq!(body, json!([
//   {
//     "id": "abb71ec2-660a-4e63-89b2-5a78fc48bdda",
//     "name": "Welcome Email",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1",
//       "key2": "value2"
//     },
//     "content_plaintext": "Hello, welcome to our service!",
//     "content_html": "<h1>Hello</h1><p>Welcome to our service!</p>",
//     "created_at": "2025-01-20T11:03:07.451777Z",
//     "updated_at": "2025-01-20T11:03:07.451777Z"
//   },
//   {
//     "id": "da9f2b2b-1812-4a28-a6f7-41f2394f78b9",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T08:58:07.022528Z",
//     "updated_at": "2025-01-21T08:58:07.022528Z"
//   },
//   {
//     "id": "5813a7c2-ba62-444c-b68c-0ce3015ea829",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:17:33.340905Z",
//     "updated_at": "2025-01-21T09:17:33.340905Z"
//   },
//   {
//     "id": "6b635691-e5c2-4820-941a-8c1f056237d3",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:18:10.510585Z",
//     "updated_at": "2025-01-21T09:18:10.510585Z"
//   },
//   {
//     "id": "88a6c439-c5b5-422e-92d5-6e255fc6e8e1",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:18:16.819217Z",
//     "updated_at": "2025-01-21T09:18:16.819217Z"
//   },
//   {
//     "id": "d0bf9b2f-93a8-4b1b-be30-704fceed45bf",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:19:48.854354Z",
//     "updated_at": "2025-01-21T09:19:48.854354Z"
//   },
//   {
//     "id": "584ec179-de88-4ad8-8998-e724bf525a0a",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:20:37.674293Z",
//     "updated_at": "2025-01-21T09:20:37.674293Z"
//   },
//   {
//     "id": "13e19165-6a6b-4cbe-b2f3-2914375c02b3",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:21:17.753334Z",
//     "updated_at": "2025-01-21T09:21:17.753334Z"
//   },
//   {
//     "id": "8ba8fea1-3c50-402d-b9d1-c2dfcf47d076",
//     "name": "Test Template",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1"
//     },
//     "content_plaintext": "Plaintext content",
//     "content_html": "<p>HTML content</p>",
//     "created_at": "2025-01-21T09:32:19.723635Z",
//     "updated_at": "2025-01-21T09:32:19.723635Z"
//   },
//   {
//     "id": "2d8c33c1-27bd-4459-90b8-919345c8a158",
//     "name": "Welcome Email 001",
//     "namespace_id": "8aba9d41-8e8f-424c-84b8-4933d54ede5e",
//     "template_data": {
//       "key1": "value1",
//       "key2": "value2"
//     },
//     "content_plaintext": "Hello, welcome to our service!",
//     "content_html": "<h1>Hello</h1><p>Welcome to our service!</p>",
//     "created_at": "2025-01-22T04:41:00.729852Z",
//     "updated_at": "2025-01-22T04:41:00.729852Z"
//   }
// ] ));
// }