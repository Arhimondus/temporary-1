use reqwest::{Error, Body};
use crate::{models::*, utils::{post, post_unauth, get, ApiError}, log};

pub async fn add_test_task(session_id: &String, r#type: &str) -> Result<bool, ApiError> {
	post::<bool>(&format!("/mobile/add-test-task/{type}"), &session_id).await
}