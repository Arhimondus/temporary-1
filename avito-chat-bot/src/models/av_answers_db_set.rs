#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvAnswers;

pub struct AvAnswersSet;

impl AvAnswersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAnswers> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAnswers> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAnswers> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAnswers>> {
        query_as::<_, AvAnswers>(r#"SELECT * FROM "av_answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, av_answers: AvAnswers) -> Result<AvAnswers> {
        query_as::<_, AvAnswers>(r#"INSERT INTO "av_answers" ("ad_id", "user_id", "question_id", "answer") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(av_answers.ad_id)
            .bind(av_answers.user_id)
            .bind(av_answers.question_id)
            .bind(av_answers.answer)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, av_answers: AvAnswers) -> Result<AvAnswers> {
        query_as::<_, AvAnswers>(r#"UPDATE "av_answers" SET "ad_id" = $1, "user_id" = $2, "question_id" = $3, "answer" = $4 WHERE  RETURNING *;"#)
            .bind(av_answers.ad_id)
            .bind(av_answers.user_id)
            .bind(av_answers.question_id)
            .bind(av_answers.answer)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "av_answers" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
