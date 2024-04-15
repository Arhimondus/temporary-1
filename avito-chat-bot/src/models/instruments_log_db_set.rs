#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentsLog;

pub struct InstrumentsLogSet;

impl InstrumentsLogSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i64) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i64>) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i64) -> Result<Option<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_warehouses_id<'e, E: PgExecutor<'e>>(executor: E, warehouses_id: i32) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE warehouse_id = $1"#)
            .bind(warehouses_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instruments_id<'e, E: PgExecutor<'e>>(executor: E, instruments_id: i32) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE instrument_id = $1"#)
            .bind(instruments_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_users_id<'e, E: PgExecutor<'e>>(executor: E, users_id: i32) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE worker_id = $1"#)
            .bind(users_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_users_id<'e, E: PgExecutor<'e>>(executor: E, users_id: i32) -> Result<Vec<InstrumentsLog>> {
        query_as::<_, InstrumentsLog>(r#"SELECT * FROM "instruments_log" WHERE controller_id = $1"#)
            .bind(users_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instruments_log: InstrumentsLog) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"INSERT INTO "instruments_log" ("id", "warehouse_id", "instrument_id", "count", "worker_id", "controller_id", "comment", "created") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(instruments_log.id)
            .bind(instruments_log.warehouse_id)
            .bind(instruments_log.instrument_id)
            .bind(instruments_log.count)
            .bind(instruments_log.worker_id)
            .bind(instruments_log.controller_id)
            .bind(instruments_log.comment)
            .bind(instruments_log.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instruments_log: InstrumentsLog) -> Result<InstrumentsLog> {
        query_as::<_, InstrumentsLog>(r#"UPDATE "instruments_log" SET "warehouse_id" = $2, "instrument_id" = $3, "count" = $4, "worker_id" = $5, "controller_id" = $6, "comment" = $7, "created" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(instruments_log.id)
            .bind(instruments_log.warehouse_id)
            .bind(instruments_log.instrument_id)
            .bind(instruments_log.count)
            .bind(instruments_log.worker_id)
            .bind(instruments_log.controller_id)
            .bind(instruments_log.comment)
            .bind(instruments_log.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "instruments_log" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
