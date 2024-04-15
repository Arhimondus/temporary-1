use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn finish_operation(session_id: &String, operation_result: &OperationResult) -> Result<bool, ApiError> {
	post_body::<bool, OperationResult>(&format!("/mobile/pass-operation/{}", operation_result.operation_id), session_id, operation_result).await
}