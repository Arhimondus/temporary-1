#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DepartmentUsers;

pub struct DepartmentUsersSet;

impl DepartmentUsersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_department_id_and_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, department_id: i32, user_id: i32) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "department_id" = $1 AND "user_id" = $2"#)
            .bind(department_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_department_id_and_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, department_id_list: Vec<i32>, user_id_list: Vec<i32>) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "department_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(department_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_department_id_and_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, department_id: i32, user_id: i32) -> Result<Option<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "department_id" = $1 AND "user_id" = $2"#)
            .bind(department_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_departments_id<'e, E: PgExecutor<'e>>(executor: E, departments_id: i32) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE department_id = $1"#)
            .bind(departments_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_users_id<'e, E: PgExecutor<'e>>(executor: E, users_id: i32) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE user_id = $1"#)
            .bind(users_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_positions_id<'e, E: PgExecutor<'e>>(executor: E, positions_id: i32) -> Result<Vec<DepartmentUsers>> {
        query_as::<_, DepartmentUsers>(r#"SELECT * FROM "department_users" WHERE position_id = $1"#)
            .bind(positions_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, department_users: DepartmentUsers) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"INSERT INTO "department_users" ("department_id", "user_id", "position_id", "intern") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(department_users.department_id)
            .bind(department_users.user_id)
            .bind(department_users.position_id)
            .bind(department_users.intern)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, department_users: DepartmentUsers) -> Result<DepartmentUsers> {
        query_as::<_, DepartmentUsers>(r#"UPDATE "department_users" SET "position_id" = $3, "intern" = $4 WHERE "department_id" = 1 AND "user_id" = 2 RETURNING *;"#)
            .bind(department_users.department_id)
            .bind(department_users.user_id)
            .bind(department_users.position_id)
            .bind(department_users.intern)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "department_users" WHERE "department_id" = 1 AND "user_id" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
