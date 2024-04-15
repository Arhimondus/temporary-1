#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Departments;

pub struct DepartmentsSet;

impl DepartmentsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Departments> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Departments> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Departments> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Departments> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Departments>> {
        query_as::<_, Departments>(r#"SELECT * FROM "departments" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, departments: Departments) -> Result<Departments> {
        query_as::<_, Departments>(r#"INSERT INTO "departments" ("id", "name") VALUES ($1, $2) RETURNING *;"#)
            .bind(departments.id)
            .bind(departments.name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, departments: Departments) -> Result<Departments> {
        query_as::<_, Departments>(r#"UPDATE "departments" SET "name" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(departments.id)
            .bind(departments.name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "departments" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
