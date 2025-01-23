use uuid::Uuid;

use backend::schema::template::dsl::*;
use backend::model::CreateTemplateRequest;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn clean_table(conn: &mut PgConnection)->(){
    match diesel::delete(template).execute(conn){
        Ok(_) => println!("Delete from table before executing test"), 
        Err(_)=>println!("Error deletion before executing test")
    };
}

pub fn insert_test_template(conn: &mut PgConnection) {
    
    clean_table(conn);

    let new_template = CreateTemplateRequest {
        name: "Test Template".to_string(),
        namespace_id: Uuid::parse_str("abb71ec2-660a-4e63-89b2-5a78fc48bdda").expect("Cannot parse UUID"),
        template_data: serde_json::from_str(r#"{"key1":"value1"}"#).unwrap(),
        content_plaintext: Some("Plaintext content".to_string()),
        content_html: "<p>HTML content</p>".to_string(),
    };

    diesel::insert_into(template)
        .values(&new_template)
        .execute(conn)
        .expect("Failed to insert test template");
}
