use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn materials(session: &String, task_id: u32) -> Result<Vec<Material>, ApiError> {
	get::<Vec<Material>>(&format!("/mobile/task-materials/{task_id}"), session).await
}