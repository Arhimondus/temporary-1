#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AvAnswers {
  pub ad_id: i64,
  pub user_id: i64,
  pub question_id: i32,
  pub answer: String,
  pub question_mnemo: Option<String>,
  pub question_name: Option<String>,
}
