#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AvOperations {
  pub id: i32,
  pub title: String,
  pub bot_message: String,
}
