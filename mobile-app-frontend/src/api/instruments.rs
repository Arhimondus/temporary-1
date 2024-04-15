use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

pub async fn instruments(session: &String, task_id: u32) -> Result<Vec<Instrument>, ApiError> {
	get::<Vec<Instrument>>(&format!("/mobile/task-instruments/{task_id}"), session).await
}