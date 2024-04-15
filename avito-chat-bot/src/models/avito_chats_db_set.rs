#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvitoChats;

pub struct AvitoChatsSet;

impl AvitoChatsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_chat_id<'e, E: PgExecutor<'e>>(&self, executor: E, chat_id: String) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "chat_id" = $1"#)
            .bind(chat_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_chat_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, chat_id_list: Vec<String>) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "chat_id" = ANY($1)"#)
            .bind(chat_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_chat_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, chat_id: String) -> Result<Option<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "chat_id" = $1"#)
            .bind(chat_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_users_id<'e, E: PgExecutor<'e>>(executor: E, users_id: i32) -> Result<Vec<AvitoChats>> {
        query_as::<_, AvitoChats>(r#"SELECT * FROM "avito_chats" WHERE linked_user_id = $1"#)
            .bind(users_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, avito_chats: AvitoChats) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"INSERT INTO "avito_chats" ("chat_id", "created", "updated", "user_id", "user_name", "user_avatar", "user_url", "last_message", "linked_user_id", "messages") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;"#)
            .bind(avito_chats.chat_id)
            .bind(avito_chats.created)
            .bind(avito_chats.updated)
            .bind(avito_chats.user_id)
            .bind(avito_chats.user_name)
            .bind(avito_chats.user_avatar)
            .bind(avito_chats.user_url)
            .bind(avito_chats.last_message)
            .bind(avito_chats.linked_user_id)
            .bind(avito_chats.messages)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, avito_chats: AvitoChats) -> Result<AvitoChats> {
        query_as::<_, AvitoChats>(r#"UPDATE "avito_chats" SET "created" = $2, "updated" = $3, "user_id" = $4, "user_name" = $5, "user_avatar" = $6, "user_url" = $7, "last_message" = $8, "linked_user_id" = $9, "messages" = $10 WHERE "chat_id" = 1 RETURNING *;"#)
            .bind(avito_chats.chat_id)
            .bind(avito_chats.created)
            .bind(avito_chats.updated)
            .bind(avito_chats.user_id)
            .bind(avito_chats.user_name)
            .bind(avito_chats.user_avatar)
            .bind(avito_chats.user_url)
            .bind(avito_chats.last_message)
            .bind(avito_chats.linked_user_id)
            .bind(avito_chats.messages)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "avito_chats" WHERE "chat_id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
