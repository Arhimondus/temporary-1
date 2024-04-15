#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Customers {
  pub id: i32,
  pub name: String,
  pub phone: Option<String>,
  pub type: i16,
  pub comment: Option<String>,
  pub address: Option<String>,
}
