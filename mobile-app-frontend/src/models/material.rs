use chrono::{DateTime, Utc};
use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use parse_display::{Display, FromStr};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Material {
	id: u32,
	operation_id: u32,
	name: String,
	count: String,
	unit: String,
}