#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AvitoChats {
  pub chat_id: String,
  pub created: chrono::DateTime<chrono::Utc>,
  pub updated: chrono::DateTime<chrono::Utc>,
  pub user_id: i32,
  pub user_name: String,
  pub user_avatar: String,
  pub user_url: String,
  pub last_message: Option<String>,
  pub linked_user_id: Option<i32>,
  pub messages: Option<serde_json::Value>,
}
