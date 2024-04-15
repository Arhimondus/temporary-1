use pub_this::pub_this;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct ChangePauseParams {
	index: usize,
	comment: Option<String>,
}