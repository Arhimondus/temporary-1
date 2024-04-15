#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::ChatAnswer;

#[derive(sqlx::FromRow, Debug)]
pub struct AvQuestions {
  pub id: i32,
  pub ad_id: i64,
  pub order_id: i32,
  pub question: String,
  pub available_answers: Option<sqlx::types::Json<Vec<ChatAnswer>>>,
  pub mnemo: Option<String>,
}
