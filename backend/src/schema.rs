// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "server_type"))]
    pub struct ServerType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tls_type"))]
    pub struct TlsType;
}

diesel::table! {
    bounce_logs (id) {
        id -> Uuid,
        contact_id -> Uuid,
        mail_id -> Text,
        campaign_id -> Nullable<Uuid>,
        at -> Timestamptz,
        kind -> Text,
        reason -> Text,
    }
}

diesel::table! {
    campaign_lists (campaign_id, list_id) {
        campaign_id -> Uuid,
        list_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    mails (id) {
        id -> Text,
        mail_message -> Text,
        contact_id -> Uuid,
        template_id -> Nullable<Uuid>,
        campaign_id -> Nullable<Uuid>,
        sent_at -> Timestamptz,
        status -> Text,
        open -> Nullable<Timestamptz>,
        clicks -> Int4,
        server_id -> Nullable<Uuid>,
        scheduled_at -> Timestamptz,
        attempts -> Int4,
        last_error -> Nullable<Text>,
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
    use diesel::sql_types::*;
    use super::sql_types::TlsType;
    use super::sql_types::ServerType;

    servers (id) {
        id -> Uuid,
        active -> Bool,
        host -> Varchar,
        smtp_username -> Varchar,
        smtp_password -> Varchar,
        namespace_id -> Uuid,
        tls_type -> TlsType,
        port -> Int2,
        server_type -> ServerType,
        aws_credentials -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        default_from_email -> Varchar,
        rate_limit -> Int4,
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

diesel::joinable!(bounce_logs -> campaigns (campaign_id));
diesel::joinable!(bounce_logs -> contacts (contact_id));
diesel::joinable!(bounce_logs -> mails (mail_id));
diesel::joinable!(campaign_lists -> campaigns (campaign_id));
diesel::joinable!(campaign_lists -> lists (list_id));
diesel::joinable!(campaign_senders -> servers (server_id));
diesel::joinable!(campaigns -> campaign_senders (campaign_senders));
diesel::joinable!(campaigns -> namespaces (namespace_id));
diesel::joinable!(campaigns -> templates (template_id));
diesel::joinable!(list_contacts -> contacts (contact_id));
diesel::joinable!(list_contacts -> lists (list_id));
diesel::joinable!(lists -> namespaces (namespace_id));
diesel::joinable!(mails -> campaigns (campaign_id));
diesel::joinable!(mails -> contacts (contact_id));
diesel::joinable!(mails -> templates (template_id));
diesel::joinable!(servers -> namespaces (namespace_id));
diesel::joinable!(templates -> namespaces (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    bounce_logs,
    campaign_lists,
    campaign_senders,
    campaigns,
    contacts,
    list_contacts,
    lists,
    mails,
    namespaces,
    servers,
    templates,
);
