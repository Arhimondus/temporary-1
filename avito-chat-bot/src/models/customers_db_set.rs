#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Customers;

pub struct CustomersSet;

impl CustomersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Customers> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Customers> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Customers> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Customers> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Customers>> {
        query_as::<_, Customers>(r#"SELECT * FROM "customers" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, customers: Customers) -> Result<Customers> {
        query_as::<_, Customers>(r#"INSERT INTO "customers" ("id", "name", "phone", "type", "comment", "address") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(customers.id)
            .bind(customers.name)
            .bind(customers.phone)
            .bind(customers.type)
            .bind(customers.comment)
            .bind(customers.address)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, customers: Customers) -> Result<Customers> {
        query_as::<_, Customers>(r#"UPDATE "customers" SET "name" = $2, "phone" = $3, "type" = $4, "comment" = $5, "address" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(customers.id)
            .bind(customers.name)
            .bind(customers.phone)
            .bind(customers.type)
            .bind(customers.comment)
            .bind(customers.address)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "customers" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
