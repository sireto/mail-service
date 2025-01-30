// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        attribute -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    list_contacts (list_id, contact_id) {
        list_id -> Uuid,
        contact_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

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

diesel::joinable!(list_contacts -> contacts (contact_id));
diesel::joinable!(list_contacts -> lists (list_id));
diesel::joinable!(lists -> namespaces (namespace_id));
diesel::joinable!(templates -> namespaces (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    list_contacts,
    lists,
    namespaces,
    templates,
);
