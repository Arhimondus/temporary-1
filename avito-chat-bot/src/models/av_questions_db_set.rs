#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvQuestions;

pub struct AvQuestionsSet;

impl AvQuestionsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvQuestions>> {
        query_as::<_, AvQuestions>(r#"SELECT * FROM "av_questions" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, av_questions: AvQuestions) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"INSERT INTO "av_questions" ("id", "ad_id", "order_id", "question", "available_questions", "mnemo") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(av_questions.id)
            .bind(av_questions.ad_id)
            .bind(av_questions.order_id)
            .bind(av_questions.question)
            .bind(av_questions.available_answers)
            .bind(av_questions.mnemo)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, av_questions: AvQuestions) -> Result<AvQuestions> {
        query_as::<_, AvQuestions>(r#"UPDATE "av_questions" SET "ad_id" = $2, "order_id" = $3, "question" = $4, "available_questions" = $5, "mnemo" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(av_questions.id)
            .bind(av_questions.ad_id)
            .bind(av_questions.order_id)
            .bind(av_questions.question)
            .bind(av_questions.available_answers)
            .bind(av_questions.mnemo)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "av_questions" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
