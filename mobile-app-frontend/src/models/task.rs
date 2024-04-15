use chrono::prelude::*;
use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use parse_display::{Display, FromStr};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::components::StampType;

use super::Operation;

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TaskPassStatus {
	Open = 0,
	Canceled = 1,
	Zero = 2,
	Partial = 3,
	Full = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TaskAcceptStatus {
	Open = 0,
	Canceled = 1,
	Zero = 2,
	Partial = 3,
	Full = 4,
}

impl From<TaskAcceptStatus> for StampType {
	fn from(value: TaskAcceptStatus) -> Self {
		match value {
			TaskAcceptStatus::Open => StampType::Info,
			TaskAcceptStatus::Canceled => StampType::Zero,
			TaskAcceptStatus::Zero => StampType::Zero,
			TaskAcceptStatus::Partial => StampType::Partial,
			TaskAcceptStatus::Full => StampType::Full,
		}
	}
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum TaskType {
	General,
	Floating,
	Mixed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Task {
	id: u32,
	date: String,
	sector_id: u32,
	start_time: Option<DateTime<Utc>>,
	end_time: Option<DateTime<Utc>>,
	closed_time: Option<DateTime<Utc>>,
	name: String,
	description: Option<String>,

	floating: bool,
	closed: bool,
	// deleted: bool,
	canceled: Option<bool>,
	testing: bool,
	repeat: bool,

	plan_salary: f32,
	pass_salary: f32,
	accept_salary: f32,

	plan_id: u32,
	sector: String,
	address: String,
	r#type: TaskType, // 0 - обычная, 1 - нормировка, 2 - смешанная
	operations: Vec<Operation>,

	pass_status: TaskPassStatus,
	accept_status: TaskAcceptStatus,

	real_start_time: Option<DateTime<Utc>>,
	real_end_time: Option<DateTime<Utc>>,
}

impl Task {
	pub fn real_start_time(&self) -> String {
		match &self.real_start_time {
			Some(utc) => {
				let converted: DateTime<Local> = DateTime::from(*utc);
				format!("{}", converted.format("%H:%M"))
			},
			None => "".to_string(),
		}
	}

	pub fn real_end_time(&self) -> String {
		match &self.real_end_time {
			Some(utc) => {
				let converted: DateTime<Local> = DateTime::from(*utc);
				format!("{}", converted.format("%H:%M"))
			},
			None => "".to_string(),
		}
	}

	pub fn start_time(&self) -> String {
		match &self.start_time {
			Some(utc) => {
				let converted: DateTime<Local> = DateTime::from(*utc);
				format!("{}", converted.format("%H:%M"))
			},
			None => "".to_string(),
		}
	}

	pub fn end_time(&self) -> String {
		match &self.end_time {
			Some(utc) => {
				let converted: DateTime<Local> = DateTime::from(*utc);
				format!("{}", converted.format("%H:%M"))
			},
			None => "".to_string(),
		}
	}

	pub fn time(&self) -> String {
		if self.r#type == TaskType::General {
			format!("{} - {}", self.start_time(), self.end_time())
		} else {
			format!("{}", self.start_time())
		}
	}

	pub fn real_time(&self) -> String {
		format!("{} - {}", self.real_start_time(), self.real_end_time())
	}
}