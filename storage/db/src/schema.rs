pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "auth_user_provider_kind"))]
    pub struct AuthUserProviderKind;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "storage_provider_kind"))]
    pub struct StorageProviderKind;
}

diesel::table! {
    auth_user (id) {
        id -> Uuid,
        time_created -> Timestamptz,
        time_modified -> Timestamptz,
        time_deleted -> Nullable<Timestamptz>,
        email -> Text,
    }
}

diesel::table! {
    auth_user_password (id) {
        id -> Uuid,
        time_created -> Timestamptz,
        password_hash -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuthUserProviderKind;

    auth_user_provider (id) {
        id -> Uuid,
        time_created -> Timestamptz,
        time_modified -> Timestamptz,
        time_deleted -> Nullable<Timestamptz>,
        provider_kind -> AuthUserProviderKind,
        provider_id -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StorageProviderKind;

    storage_object (id) {
        id -> Uuid,
        time_created -> Timestamptz,
        time_modified -> Timestamptz,
        time_deleted -> Nullable<Timestamptz>,
        provider_kind -> StorageProviderKind,
        provider_path -> Text,
        mime_type -> Text,
    }
}

diesel::joinable!(auth_user_password -> auth_user (user_id));
diesel::joinable!(auth_user_provider -> auth_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(auth_user, auth_user_password, auth_user_provider,);
