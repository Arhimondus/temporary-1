use pub_this::pub_this;
use reqwest::{Error, Body};
use serde::{Serialize, Deserialize};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError}, log};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct TasksCount {
	new_tasks: u8,
	taken_tasks: u8,
	finished_tasks: u8,
}

pub async fn tasks_count(session: &String) -> Result<TasksCount, ApiError> {
	get::<TasksCount>("/mobile/tasks-count", session).await
}