mod fixtures;
mod test_env;
use test_env::TEST_ENV;
use backend::model::GetTemplateResponse;

use fixtures::insert_test_template;


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

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "Test Template");
}

