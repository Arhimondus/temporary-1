#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct DepartmentUsers {
  pub department_id: i32,
  pub user_id: i32,
  pub position_id: i32,
  pub intern: bool,
}
