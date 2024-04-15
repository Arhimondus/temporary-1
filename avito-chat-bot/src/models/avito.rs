#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Avito {
  pub access_token: Option<String>,
  pub expire_in: Option<i32>,
  pub token_type: Option<String>,
  pub modified: Option<chrono::DateTime<chrono::Utc>>,
  pub chats_count: i16,
  pub new_update: bool,
}
