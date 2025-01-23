// @generated automatically by Diesel CLI.

diesel::table! {
    BounceLog (id) {
        id -> Text,
        bounce_reason -> Text,
        bounce_type -> Text,
        bounced_at -> Timestamp,
        campaign_id -> Text,
        contact_id -> Text,
    }
}

diesel::table! {
    Campaign (id) {
        id -> Text,
        status -> Text,
        campaign_sender_id -> Text,
        created_at -> Timestamp,
        name -> Text,
        namespace_id -> Text,
        scheduled_at -> Timestamp,
        template_id -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    CampaignSender (id) {
        id -> Text,
        created_at -> Timestamp,
        from_email -> Text,
        from_name -> Text,
        server_id -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    Contact (id) {
        id -> Text,
        email -> Text,
        attribute -> Jsonb,
        created_at -> Timestamp,
        first_name -> Text,
        last_name -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    List (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        namespace_id -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ListContact (list_id, contact_id) {
        contact_id -> Text,
        created_at -> Timestamp,
        list_id -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    Mail (id) {
        id -> Text,
        message -> Text,
        status -> Text,
        campaign_id -> Text,
        contact_id -> Text,
        sent_at -> Timestamp,
        template_id -> Text,
    }
}

diesel::table! {
    Namespace (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    Server (id) {
        id -> Text,
        host -> Text,
        created_at -> Timestamp,
        namespace_id -> Text,
        smtp_password -> Text,
        smtp_username -> Text,
        updated_at -> Timestamp,
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
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
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

diesel::joinable!(BounceLog -> Campaign (campaign_id));
diesel::joinable!(BounceLog -> Contact (contact_id));
diesel::joinable!(Campaign -> CampaignSender (campaign_sender_id));
diesel::joinable!(CampaignSender -> Server (server_id));
diesel::joinable!(ListContact -> Contact (contact_id));
diesel::joinable!(ListContact -> List (list_id));
diesel::joinable!(Mail -> Campaign (campaign_id));
diesel::joinable!(Mail -> Contact (contact_id));
diesel::joinable!(Template -> Namespace (namespace_id));
diesel::joinable!(template -> Namespace (namespace_id));
diesel::joinable!(templates -> namespaces (namespace_id));

diesel::allow_tables_to_appear_in_same_query!(
    BounceLog,
    Campaign,
    CampaignSender,
    Contact,
    List,
    ListContact,
    Mail,
    Namespace,
    Server,
    Template,
    _prisma_migrations,
    namespaces,
    template,
    templates,
);
