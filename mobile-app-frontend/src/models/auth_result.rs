use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

use super::User;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct AuthResult {
	session_id: String,
	user: User,
}