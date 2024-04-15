#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct DieselSchemaMigrations {
  pub version: String,
  pub run_on: chrono::NaiveDateTime,
}
