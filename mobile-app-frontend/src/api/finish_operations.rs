use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn finish_operations(session_id: &String, operation_results: &Vec<OperationResult>) -> Vec<Result<bool, ApiError>> {
	let mut api_errors: Vec<Result<bool, ApiError>> = vec![];
	for operation_result in operation_results.into_iter() {
		api_errors.push(post_body::<bool, OperationResult>(&format!("/mobile/pass-operation/{}", operation_result.operation_id), session_id, operation_result).await);
	}
	api_errors
}