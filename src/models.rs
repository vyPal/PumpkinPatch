use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub oidc_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub oidc_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = crate::schema::plugins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub author_id: i32,
    pub publish_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub download_count: i32,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(Plugin))]
#[diesel(table_name = crate::schema::plugin_versions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginVersion {
    pub id: i32,
    pub plugin_id: i32,
    pub version_number: String,
    pub release_date: NaiveDateTime,
    pub download_count: i32,
    pub windows_url: Option<String>,
    pub linux_url: Option<String>,
    pub macos_url: Option<String>,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Plugin))]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub plugin_id: i32,
    pub user_id: i32,
    pub content: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Plugin))]
#[diesel(table_name = crate::schema::ratings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rating {
    pub id: i32,
    pub plugin_id: i32,
    pub user_id: i32,
    pub rating: i32,
}
