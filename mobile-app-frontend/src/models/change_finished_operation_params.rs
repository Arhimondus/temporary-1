use pub_this::pub_this;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct FinishedOperationParams {
	count: f32,
	comment: Option<String>,
}