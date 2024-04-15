use reqwest::{Error, Body};
use crate::{models::*, utils::{post, post_unauth, get, ApiError}, log};

pub async fn accept_task(session_id: &String, task_id: u32) -> Result<bool, ApiError> {
	post::<bool>(&format!("/mobile/accept-task/{task_id}"), &session_id).await
}