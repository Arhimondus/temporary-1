#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AvQuestionnaires {
  pub user_id: i64,
  pub ad_id: i64,
  pub current_question_id: Option<i32>,
  pub chat_id: String,
}
