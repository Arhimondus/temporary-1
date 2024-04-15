#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvOperations;

pub struct AvOperationsSet;

impl AvOperationsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvOperations>> {
        query_as::<_, AvOperations>(r#"SELECT * FROM "av_operations" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, av_operations: AvOperations) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"INSERT INTO "av_operations" ("id", "title", "bot_message") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(av_operations.id)
            .bind(av_operations.title)
            .bind(av_operations.bot_message)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, av_operations: AvOperations) -> Result<AvOperations> {
        query_as::<_, AvOperations>(r#"UPDATE "av_operations" SET "title" = $2, "bot_message" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(av_operations.id)
            .bind(av_operations.title)
            .bind(av_operations.bot_message)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "av_operations" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
