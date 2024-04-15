use reqwest::{Error, Body};
use crate::{models::*, utils::{post, post_unauth, get, ApiError}, log};

pub async fn decline_task(session_id: &String, task_id: u32) -> Result<bool, ApiError> {
	post::<bool>(&format!("/mobile/decline-task/{task_id}"), &session_id).await
}