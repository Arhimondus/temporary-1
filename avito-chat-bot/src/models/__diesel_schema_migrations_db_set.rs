#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DieselSchemaMigrations;

pub struct DieselSchemaMigrationsSet;

impl DieselSchemaMigrationsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_version<'e, E: PgExecutor<'e>>(&self, executor: E, version: String) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "version" = $1"#)
            .bind(version)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_version_list<'e, E: PgExecutor<'e>>(&self, executor: E, version_list: Vec<String>) -> Result<Vec<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "version" = ANY($1)"#)
            .bind(version_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_version_optional<'e, E: PgExecutor<'e>>(&self, executor: E, version: String) -> Result<Option<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "version" = $1"#)
            .bind(version)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DieselSchemaMigrations>> {
        query_as::<_, DieselSchemaMigrations>(r#"SELECT * FROM "__diesel_schema_migrations" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, __diesel_schema_migrations: DieselSchemaMigrations) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"INSERT INTO "__diesel_schema_migrations" ("version", "run_on") VALUES ($1, $2) RETURNING *;"#)
            .bind(__diesel_schema_migrations.version)
            .bind(__diesel_schema_migrations.run_on)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, __diesel_schema_migrations: DieselSchemaMigrations) -> Result<DieselSchemaMigrations> {
        query_as::<_, DieselSchemaMigrations>(r#"UPDATE "__diesel_schema_migrations" SET "run_on" = $2 WHERE "version" = 1 RETURNING *;"#)
            .bind(__diesel_schema_migrations.version)
            .bind(__diesel_schema_migrations.run_on)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "__diesel_schema_migrations" WHERE "version" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
