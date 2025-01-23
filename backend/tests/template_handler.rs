use std::env;

use diesel_async::{ AsyncConnection, AsyncPgConnection, SimpleAsyncConnection };

use serde::{ Serialize, Deserialize };
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::NaiveDateTime;

use backend::model::DeleteTemplateResponse;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTemplateRequest {
    pub name: String,
    pub template_data: Value,
    pub content_html: String,
    pub content_plaintext: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTemplateResponse {
    #[schema(value_type = String, example = "825b5155-2c6e-4262-bf4c-e0bf2323e565")]
    pub id: Uuid,

    pub name: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: NaiveDateTime
}

// Set up your test database connection.
async fn setup_test_db() -> AsyncPgConnection {
    let db_url = env::var("DATABASE_URL").unwrap();
    AsyncPgConnection::establish(db_url.as_str()).await.unwrap()
}

// Cleanup the test database after the test.
async fn cleanup_test_db(conn: &mut AsyncPgConnection) {
    conn.batch_execute("DROP TABLE __test_mail_service").await.unwrap()
}

#[tokio::test]
async fn test_template_update() {
    // let mut conn = setup_test_db().await;
    let url = "http://localhost:8000/api/templates/ca088c8c-385b-436e-9d49-3b39241a9b22";

    let payload = UpdateTemplateRequest {
        name: "Test Template".to_string(),
        template_data: serde_json::json!({"Hank": "Walt"}),
        content_html: "<html><body><h1>Test Template Template</h1></body></html>".to_string(),
        content_plaintext: "Test Template".to_string(),
    };

    let response = reqwest::Client::new()
        .patch(url)
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    let status_code = response.status();
    println!("Response status: {}", status_code);
    assert!(response.status().is_success());
    // assert_eq!(response.status(), 200);

    let response_body: UpdateTemplateResponse = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert_eq!(response_body.id.to_string(), "ca088c8c-385b-436e-9d49-3b39241a9b22");
    assert_eq!(response_body.name, "Test Template");
    // assert_eq!(response_body.updated_at, "2023-01-01T00:00:00Z");

    // Clean up the test database
    // cleanup_test_db(&mut conn).await;
}

#[tokio::test]
async fn test_template_delete() {
    let url = "http://localhost:8000/api/templates/b6b9b7fc-af59-41e8-8454-27cb857a56c8";

    let response = reqwest::Client::new()
        .delete(url)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(response.status(), 200);

    let response_body: DeleteTemplateResponse = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert_eq!(response_body.id.to_string(), "b6b9b7fc-af59-41e8-8454-27cb857a56c8");
    assert_eq!(response_body.name, "Test Template");
    // assert_eq!(response_body.updated_at, "2023-01-01T00:00:00Z");
}