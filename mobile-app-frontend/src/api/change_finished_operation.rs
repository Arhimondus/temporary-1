use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn change_finished_operation(session_id: &String, task_id: u32, operation_id: u32, count: f32, comment: Option<String>) -> Result<bool, ApiError> {
	post_body::<bool, FinishedOperationParams>(&format!("/mobile/change-finished-operation/{}", operation_id), session_id, &FinishedOperationParams {
		count,
		comment,
	}).await
}