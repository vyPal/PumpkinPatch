// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        plugin_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        content -> Text,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    plugin_versions (id) {
        id -> Int4,
        plugin_id -> Nullable<Int4>,
        version_number -> Text,
        release_date -> Nullable<Timestamp>,
        download_count -> Nullable<Int4>,
        windows_url -> Nullable<Text>,
        linux_url -> Nullable<Text>,
        macos_url -> Nullable<Text>,
    }
}

diesel::table! {
    plugins (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        author_id -> Nullable<Int4>,
        publish_date -> Nullable<Timestamp>,
        last_update_date -> Nullable<Timestamp>,
        download_count -> Nullable<Int4>,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        plugin_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        rating -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(comments -> plugins (plugin_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(plugin_versions -> plugins (plugin_id));
diesel::joinable!(plugins -> users (author_id));
diesel::joinable!(ratings -> plugins (plugin_id));
diesel::joinable!(ratings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    plugin_versions,
    plugins,
    ratings,
    users,
);
