#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct InstrumentsLog {
  pub id: i64,
  pub warehouse_id: i32,
  pub instrument_id: i32,
  pub count: i16,
  pub worker_id: Option<i32>,
  pub controller_id: i32,
  pub comment: Option<String>,
  pub created: chrono::NaiveDateTime,
}
