use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn taken_tasks(session: &String) -> Result<Vec<Task>, ApiError> {
	get::<Vec<Task>>("/mobile/taken-tasks", session).await
}