use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn change_pause(session_id: &String, task_id: u32, operation_id: u32, index: usize, comment: Option<String>) -> Result<bool, ApiError> {
	post_body::<bool, ChangePauseParams>(&format!("/mobile/change-pause/{}", operation_id), session_id, &ChangePauseParams {
		index,
		comment,
	}).await
}