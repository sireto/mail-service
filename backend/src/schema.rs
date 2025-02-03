// @generated automatically by Diesel CLI.

diesel::table! {
    campaign_senders (id) {
        id -> Uuid,
        server_id -> Uuid,
        from_name -> Varchar,
        from_email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    campaigns (id) {
        id -> Uuid,
        campaign_name -> Varchar,
        template_id -> Uuid,
        namespace_id -> Uuid,
        status -> Varchar,
        campaign_senders -> Nullable<Uuid>,
        scheduled_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
    servers (id) {
        id -> Uuid,
        host -> Varchar,
        smtp_username -> Varchar,
        smtp_password -> Varchar,
        namespace_id -> Uuid,
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

diesel::joinable!(campaign_senders -> servers (server_id));
diesel::joinable!(campaigns -> campaign_senders (campaign_senders));
diesel::joinable!(campaigns -> namespaces (namespace_id));
diesel::joinable!(campaigns -> templates (template_id));
diesel::joinable!(list_contacts -> contacts (contact_id));
diesel::joinable!(list_contacts -> lists (list_id));
diesel::joinable!(lists -> namespaces (namespace_id));
diesel::joinable!(servers -> namespaces (namespace_id));
diesel::joinable!(templates -> namespaces (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    campaign_senders,
    campaigns,
    contacts,
    list_contacts,
    lists,
    namespaces,
    servers,
    templates,
);
