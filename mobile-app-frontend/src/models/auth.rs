use pub_this::pub_this;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct Auth {
	login: String,
	password: String,
}