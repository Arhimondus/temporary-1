#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvQuestionnaires;

pub struct AvQuestionnairesSet;

impl AvQuestionnairesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_and_ad_id_and_chat_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: i32, ad_id: i32, chat_id: String) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "user_id" = $1 AND "ad_id" = $2 AND "chat_id" = $3"#)
            .bind(user_id)
            .bind(ad_id)
            .bind(chat_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_and_ad_id_and_chat_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<i32>, ad_id_list: Vec<i32>, chat_id_list: Vec<String>) -> Result<Vec<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "user_id" = ANY($1) AND "ad_id" = ANY($2) AND "chat_id" = ANY($3)"#)
            .bind(user_id_list)
            .bind(ad_id_list)
            .bind(chat_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_and_ad_id_and_chat_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: i32, ad_id: i32, chat_id: String) -> Result<Option<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "user_id" = $1 AND "ad_id" = $2 AND "chat_id" = $3"#)
            .bind(user_id)
            .bind(ad_id)
            .bind(chat_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestionnaires>> {
        query_as::<_, AvQuestionnaires>(r#"SELECT * FROM "av_questionnaires" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, av_questionnaires: AvQuestionnaires) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"INSERT INTO "av_questionnaires" ("user_id", "ad_id", "current_question_id", "chat_id") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(av_questionnaires.user_id)
            .bind(av_questionnaires.ad_id)
            .bind(av_questionnaires.current_question_id)
            .bind(av_questionnaires.chat_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, av_questionnaires: AvQuestionnaires) -> Result<AvQuestionnaires> {
        query_as::<_, AvQuestionnaires>(r#"UPDATE "av_questionnaires" SET "current_question_id" = $3 WHERE "user_id" = 1 AND "ad_id" = 2 AND "chat_id" = 4 RETURNING *;"#)
            .bind(av_questionnaires.user_id)
            .bind(av_questionnaires.ad_id)
            .bind(av_questionnaires.current_question_id)
            .bind(av_questionnaires.chat_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "av_questionnaires" WHERE "user_id" = 1 AND "ad_id" = 2 AND "chat_id" = 3"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
