use pub_this::pub_this;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct Plan {
	id: u32,
	task_id: u32,
	user_id: u32,
	accepted: Option<bool>,
	comment: Option<String>,
	accept_time: Option<String>,
}