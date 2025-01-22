// @generated automatically by Diesel CLI.

diesel::table! {
    Namespace (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    Template (id) {
        id -> Uuid,
        namespace_id -> Uuid,
        name -> Text,
        template_data -> Jsonb,
        content_plaintext -> Nullable<Text>,
        content_html -> Text,
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
    template (id) {
        id -> Uuid,
        namespace_id -> Uuid,
        name -> Text,
        template_data -> Jsonb,
        content_plaintext -> Nullable<Text>,
        content_html -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(Template -> Namespace (namespace_id));
diesel::joinable!(template -> Namespace (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    Namespace,
    Template,
    namespaces,
    template,
);
