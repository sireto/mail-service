use std::env;

mod fixtures;

use backend::tests::setup_test_environment;

use fixtures::{ 
    delete_test_template_by_id, 
    get_test_template_by_id, 
    insert_test_template 
};
use backend::model::{ DeleteTemplateResponse, UpdateTemplateResponse };
use backend::route::create_router;

use backend::establish_connection;

use std::net::SocketAddr;

use diesel_async::{ AsyncConnection, AsyncPgConnection, SimpleAsyncConnection };


async fn start_server () {
    let app = create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Spawn the server in the background...
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });
}

// // Set up your test database connection.
// async fn setup_test_db() -> AsyncPgConnection {
//     let db_url = env::var("DATABASE_URL").unwrap();
//     AsyncPgConnection::establish(db_url.as_str()).await.unwrap()
// }

// // Cleanup the test database after the test.
// async fn cleanup_test_db(conn: &mut AsyncPgConnection) {
//     conn.batch_execute("DROP TABLE __test_mail_service").await.unwrap()
// }

#[tokio::test]

async fn test_update_templates() {
    setup_test_environment();
    let pool = establish_connection();

    let conn = &mut pool.get().expect("Failed to get DB connection");

    start_server().await;

    let inserted_template = insert_test_template(conn);

    println!("Template created and inserted successfully");

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let update_request_url = format!("http://localhost:9000/api/templates/{}", inserted_template.id);
    // test the udpate template handler...
    let response = reqwest::Client::new()
        .patch(update_request_url)
        .json(&serde_json::json!({
            "name": "Updated Template",
            "template_data": {
                "key_updated": "val_updated_too"
            },
            "content_html": "<html><body><h1>Updated Template</h1></body></html>",
            "content_plaintext": "Updated Template"
        }))
        .send()
        .await
        .expect("Failed to send request");

    // assertions for the updating the template...
    assert!(response.status().is_success());

    let updated_template: UpdateTemplateResponse = response.json().await.expect("Failed to parse response");

    assert_eq!(updated_template.name, "Updated Template");
    assert_eq!(updated_template.id, inserted_template.id);  // assertion that the created template is updated...

    // Clear the updated record (test deletion)...
    let deleted_template: DeleteTemplateResponse = delete_test_template_by_id(conn, updated_template.id);

    assert_eq!(deleted_template.id, updated_template.id);
        

    // now test if the template is deleted...
    let response = get_test_template_by_id(conn, deleted_template.id);

    println!("THe response after deletion ===> {:?}", response);

    assert!(matches!(response, Err(diesel::result::Error::NotFound)),
        "Expected NotFound error but go: {:?}", response);  // add the error message to the assertion...

    assert!(deleted_template.id == inserted_template.id);   
    assert!(deleted_template.id == updated_template.id);
}