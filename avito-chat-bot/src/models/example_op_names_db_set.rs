#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ExampleOpNames;

pub struct ExampleOpNamesSet;

impl ExampleOpNamesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ExampleOpNames> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ExampleOpNames> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ExampleOpNames> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ExampleOpNames>> {
        query_as::<_, ExampleOpNames>(r#"SELECT * FROM "example_op_names" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, example_op_names: ExampleOpNames) -> Result<ExampleOpNames> {
        query_as::<_, ExampleOpNames>(r#"INSERT INTO "example_op_names" ("name") VALUES ($1) RETURNING *;"#)
            .bind(example_op_names.name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, example_op_names: ExampleOpNames) -> Result<ExampleOpNames> {
        query_as::<_, ExampleOpNames>(r#"UPDATE "example_op_names" SET "name" = $1 WHERE  RETURNING *;"#)
            .bind(example_op_names.name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "example_op_names" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
