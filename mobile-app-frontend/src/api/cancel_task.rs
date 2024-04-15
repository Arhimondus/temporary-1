use reqwest::{Error, Body};
use crate::{models::*, utils::{post_unauth, get, ApiError, post_body}};

pub async fn cancel_task(session_id: &String, task_id: u32, comment: &str) -> Result<bool, ApiError> {
	post_body::<bool, CancelTaskBody>(&format!("/mobile/cancel-task/{task_id}"), &session_id, &CancelTaskBody { comment: comment.to_string() }).await
}