use pub_this::pub_this;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct User {
	id: u32,
	phone: String,
	name: String,
	specialization_name: String,
	specialization_id: u32,
	intern: Option<bool>,
	debug_mode: bool,
}