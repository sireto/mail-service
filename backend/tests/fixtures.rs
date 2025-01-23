use diesel::prelude::*;
use backend::schema::templates::dsl::*;
use backend::model::{ CreateTemplateRequest, DeleteTemplateResponse, Template };
use uuid::Uuid;

pub fn insert_test_template(conn: &mut PgConnection) -> Template {
    let new_template = CreateTemplateRequest {
        name: "Test Template".to_string(),
        namespace_id: Uuid::parse_str("8790f149-7830-483c-ba50-6ba8b60ac965").expect("Cannot parse UUID"),
        template_data: serde_json::from_str(r#"{"key1":"value1"}"#).unwrap(),
        content_plaintext: Some("Plaintext content".to_string()),
        content_html: "<p>HTML VERY NEW CONTENT</p>".to_string(),
    };

    let inserted_template: Template = diesel::insert_into(templates)
        .values(&new_template)
        .get_result(conn)
        .expect("Failed to insert test template");

    inserted_template
}

pub fn delete_test_template(conn: &mut PgConnection) {
    diesel::delete(templates)
        .execute(conn)
        .expect("Failed to insert test template");
}

pub fn delete_test_template_by_id(conn: &mut PgConnection, template_id: Uuid) -> DeleteTemplateResponse {
    let deleted_template = diesel::delete(templates.filter(id.eq(template_id)))
        .returning((id, name))
        .get_result::<DeleteTemplateResponse>(conn)
        .expect("Failed to delete test template");

    deleted_template
}

pub fn get_test_template_by_id(conn: &mut PgConnection, template_id: Uuid) -> Result<Template, diesel::result::Error> {
    templates
        .filter(id.eq(template_id))
        .first::<Template>(conn)
}