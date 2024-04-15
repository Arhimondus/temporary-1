#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AvAds {
  pub id: i64,
  pub title: String,
  pub trigger_message: String,
  pub last_message: String,
}
