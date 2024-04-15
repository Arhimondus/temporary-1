#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Avito;

pub struct AvitoSet;

impl AvitoSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Avito> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Avito> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Avito> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Avito>> {
        query_as::<_, Avito>(r#"SELECT * FROM "avito" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, avito: Avito) -> Result<Avito> {
        query_as::<_, Avito>(r#"INSERT INTO "avito" ("access_token", "expire_in", "token_type", "modified", "chats_count", "new_update") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(avito.access_token)
            .bind(avito.expire_in)
            .bind(avito.token_type)
            .bind(avito.modified)
            .bind(avito.chats_count)
            .bind(avito.new_update)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, avito: Avito) -> Result<Avito> {
        query_as::<_, Avito>(r#"UPDATE "avito" SET "access_token" = $1, "expire_in" = $2, "token_type" = $3, "modified" = $4, "chats_count" = $5, "new_update" = $6 WHERE  RETURNING *;"#)
            .bind(avito.access_token)
            .bind(avito.expire_in)
            .bind(avito.token_type)
            .bind(avito.modified)
            .bind(avito.chats_count)
            .bind(avito.new_update)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "avito" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
