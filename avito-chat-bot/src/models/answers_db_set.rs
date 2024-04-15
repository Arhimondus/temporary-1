#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Answers;

pub struct AnswersSet;

impl AnswersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_and_question_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: i32, question_id: i32) -> Result<Answers> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "user_id" = $1 AND "question_id" = $2"#)
            .bind(user_id)
            .bind(question_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_and_question_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<i32>, question_id_list: Vec<i32>) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "user_id" = ANY($1) AND "question_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(question_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_and_question_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: i32, question_id: i32) -> Result<Option<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "user_id" = $1 AND "question_id" = $2"#)
            .bind(user_id)
            .bind(question_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Answers> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Answers> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Answers> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_users_id<'e, E: PgExecutor<'e>>(executor: E, users_id: i32) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE user_id = $1"#)
            .bind(users_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_questions_id<'e, E: PgExecutor<'e>>(executor: E, questions_id: i32) -> Result<Vec<Answers>> {
        query_as::<_, Answers>(r#"SELECT * FROM "answers" WHERE question_id = $1"#)
            .bind(questions_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, answers: Answers) -> Result<Answers> {
        query_as::<_, Answers>(r#"INSERT INTO "answers" ("user_id", "question_id", "yes", "comment", "approved") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(answers.user_id)
            .bind(answers.question_id)
            .bind(answers.yes)
            .bind(answers.comment)
            .bind(answers.approved)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, answers: Answers) -> Result<Answers> {
        query_as::<_, Answers>(r#"UPDATE "answers" SET "yes" = $3, "comment" = $4, "approved" = $5 WHERE "user_id" = 1 AND "question_id" = 2 RETURNING *;"#)
            .bind(answers.user_id)
            .bind(answers.question_id)
            .bind(answers.yes)
            .bind(answers.comment)
            .bind(answers.approved)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "answers" WHERE "user_id" = 1 AND "question_id" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
