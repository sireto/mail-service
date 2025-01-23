mod fixtures;
mod test_env;
use test_env::TEST_ENV;
use backend::model::{GetTemplateResponse,CreateTemplateResponse, Template};
use backend::schema::template::dsl::*;
use diesel::prelude::*;

use fixtures::insert_test_template;
use fixtures::clean_table;


#[tokio::test]
async fn test_get_templates() {
    // Access the global test environment
    let env = &*TEST_ENV;

    // Insert a test template
    let conn = &mut env.pool.get().expect("Failed to get DB connection");
    insert_test_template(conn);

    // Test the handler
    let response = reqwest::Client::new()
        .get(format!("http://{}/api/templates", env.addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    // Deserialize the response into Vec<GetTemplateResponse>
    let templates: Vec<GetTemplateResponse> = response.json().await.expect("Failed to parse response");
    assert_eq!(templates[0].name, "Test Template");
}


#[tokio::test]
async fn test_create_template() {
    // Access the global test environment
    let env = &*TEST_ENV;

    let conn = &mut env.pool.get().expect("Failed to get DB connection");
    
    clean_table(conn);

    // Define the payload for the new template
    let payload = serde_json::json!({
        "name": "Test Template",
        "namespace_id": "abb71ec2-660a-4e63-89b2-5a78fc48bdda",
        "template_data": {
            "key1": "value1"
        },
        "content_plaintext": "Plaintext content",
        "content_html": "<p>HTML content</p>"
    });

    // Send the POST request to create a new template
    let response = reqwest::Client::new()
        .post(format!("http://{}/api/templates", env.addr))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    // Assert the response status is success
    assert!(response.status().is_success());

    // Deserialize the response into CreateTemplateResponse
    let created_template: CreateTemplateResponse = response
        .json()
        .await
        .expect("Failed to parse response");

    // Assert the response contains expected values
    assert_eq!(created_template.name, "Test Template");
    assert!(created_template.id.len() > 0);
    assert!(created_template.created_at.timestamp() > 0);

}