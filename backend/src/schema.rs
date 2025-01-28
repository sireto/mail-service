// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Uuid,
        namespace_id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    namespaces (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    templates (id) {
        id -> Uuid,
        namespace_id -> Uuid,
        name -> Varchar,
        template_data -> Jsonb,
        content_plaintext -> Nullable<Text>,
        content_html -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(lists -> namespaces (namespace_id));
diesel::joinable!(templates -> namespaces (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    namespaces,
    templates,
);
