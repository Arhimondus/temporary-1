use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct CancelTaskBody {
	comment: String,
}