#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Instruments;

pub struct InstrumentsSet;

impl InstrumentsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instruments>> {
        query_as::<_, Instruments>(r#"SELECT * FROM "instruments" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instruments: Instruments) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"INSERT INTO "instruments" ("id", "name", "deleted") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(instruments.id)
            .bind(instruments.name)
            .bind(instruments.deleted)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instruments: Instruments) -> Result<Instruments> {
        query_as::<_, Instruments>(r#"UPDATE "instruments" SET "name" = $2, "deleted" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(instruments.id)
            .bind(instruments.name)
            .bind(instruments.deleted)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "instruments" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
