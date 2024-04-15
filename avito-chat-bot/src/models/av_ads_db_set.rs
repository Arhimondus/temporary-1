#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AvAds;

pub struct AvAdsSet;

impl AvAdsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ad_id<'e, E: PgExecutor<'e>>(&self, executor: E, ad_id: i64) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "ad_id" = $1"#)
            .bind(ad_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ad_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ad_id_list: Vec<i64>) -> Result<Vec<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "ad_id" = ANY($1)"#)
            .bind(ad_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ad_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ad_id: i64) -> Result<Option<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "ad_id" = $1"#)
            .bind(ad_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_task_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_task_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_task_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_login<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_login_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_login_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AvAds>> {
        query_as::<_, AvAds>(r#"SELECT * FROM "av_ads" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, av_ads: AvAds) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"INSERT INTO "av_ads" ("ad_id", "title", "trigger_message", "last_message") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(av_ads.id)
            .bind(av_ads.title)
            .bind(av_ads.trigger_message)
            .bind(av_ads.last_message)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, av_ads: AvAds) -> Result<AvAds> {
        query_as::<_, AvAds>(r#"UPDATE "av_ads" SET "title" = $2, "trigger_message" = $3, "last_message" = $4 WHERE "ad_id" = 1 RETURNING *;"#)
            .bind(av_ads.id)
            .bind(av_ads.title)
            .bind(av_ads.trigger_message)
            .bind(av_ads.last_message)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "av_ads" WHERE "ad_id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
