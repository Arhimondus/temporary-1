#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Answers {
  pub user_id: i32,
  pub question_id: i32,
  pub yes: bool,
  pub comment: Option<String>,
  pub approved: bool,
}
