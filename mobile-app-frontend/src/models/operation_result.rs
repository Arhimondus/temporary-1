use chrono::{Utc, DateTime, Local, Duration};
use pub_this::pub_this;
use serde::ser::Error;
use serde::{Deserialize, Serialize};

use crate::state::{State, ActiveTask};
use crate::utils::{log_str, relog};

use super::Operation;
use super::Pause;
// use yew::Properties;
// use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct OperationResult {
	operation_id: u32,
	count: Option<f32>,
	comment: Option<String>,
	start_time: Option<DateTime<Utc>>,
	end_time: Option<DateTime<Utc>>,
	delta: u32,
	total_delta: u32,
	// success_load: bool,
	status: OperationResultStatus,
	pauses: Vec<Pause>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperationResultStatus {
	Open,
	Pass,
	Done(OperationDoneStatus),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperationDoneStatus {
	// StartUpload,
	Uploading,
	Successfull,
	Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CurrentOperationStatus {
	NotActive,
	Running,
	Pause,
}

impl OperationResult {
	pub fn current_operation_status(&mut self, state: &State) -> CurrentOperationStatus {
		if !self.is_current(state) {
			CurrentOperationStatus::NotActive
		} else {
			if let Some(pause) = self.pauses.last() {
				if pause.resume_time.is_none() {
					CurrentOperationStatus::Pause
				} else {
					CurrentOperationStatus::Running	
				}
			} else {
				CurrentOperationStatus::Running
			}
		}
	}

	pub fn pass(&mut self) {
		self.status = OperationResultStatus::Pass;
	}

	pub fn upload(&mut self) {
		self.status = OperationResultStatus::Done(OperationDoneStatus::Uploading);
	}

	pub fn upload_successfull(&mut self) {
		self.status = OperationResultStatus::Done(OperationDoneStatus::Successfull);
	}
	
	pub fn upload_failure(&mut self) {
		self.status = OperationResultStatus::Done(OperationDoneStatus::Failure);
	}
	
	pub fn time(&self) -> String {
		let local_start_time: DateTime<Local> = DateTime::from(self.start_time.unwrap());
		let local_end_time: DateTime<Local> = DateTime::from(self.end_time.unwrap());
		
		format!("{} - {}", local_start_time.format("%H:%M"), local_end_time.format("%H:%M"))
	}

	// pub fn duration_with_pauses() -> Duration {
	// 	let Some(start_time) = self.start_time else { return Duration::zero() };
	// }

	pub fn duration_num(&self) -> Duration {
		let Some(start_time) = self.start_time else { return Duration::zero() };
		let local_start_time: DateTime<Local> = DateTime::from(start_time);
		let local_end_time: DateTime<Local> = DateTime::from(self.end_time.unwrap_or_else(|| {
			// log_str("+___123");
			if let Some(pause) = self.pauses.last() {
				// log_str("+___124");
				if pause.resume_time.is_none() {
					// log_str("+___125");
					pause.pause_time
				} else {
					// log_str("+___126");
					Utc::now()
				}
			} else {
				// log_str("+___127");
				Utc::now()
			}
		}));

		// log_str(&format!("start_time {}", start_time));
		// log_str(&format!("local_start_time {}", local_start_time));
		// log_str(&format!("local_end_time {}", local_end_time));

		let mut pauses_time_duration = Duration::zero();
		let full_pauses = self.pauses.iter().filter(|it| it.resume_time.is_some());
		for pause in full_pauses {
			pauses_time_duration = pauses_time_duration + (pause.resume_time.unwrap() - pause.pause_time);
		}

		// log_str(&format!("pauses_time_duration {}", pauses_time_duration));

		let duration = local_end_time.naive_local() - local_start_time.naive_local() - pauses_time_duration;

		// log_str(&format!("duration {}", duration));

		duration
	}

	pub fn duration(&self) -> String {
		let duration = self.duration_num();
		let minutes = duration.num_minutes();
		let seconds = duration.num_seconds() - minutes *  60;
		
		format!("{} мин {} сек", minutes, seconds)
	}

	pub fn get_operation_from_task(&self, active_task: &ActiveTask) -> Operation {
		active_task.task.operations.iter().find(|o| o.id == self.operation_id).unwrap().clone()
	}

	pub fn get_operation(&self, state: &State) -> Operation {
		let Some(active_task) = state.active_task.as_ref() else { panic!("Получить операцию можно только, если она в активной задаче")};
		active_task.task.operations.iter().find(|o| o.id == self.operation_id).unwrap().clone()
	}

	pub fn duration_test(&self) -> String {
		let self_start_time = self.start_time;
		let start_time = self.start_time.unwrap_or(Utc::now());
		let local_start_time: DateTime<Local> = DateTime::from(start_time);
		let self_end_time = self.end_time;
		let local_end_time: DateTime<Local> = DateTime::from(self_end_time.
			unwrap_or_else(|| {
			log_str("+___123");
			if let Some(pause) = self.pauses.last() {
				log_str("+___124");
				if pause.resume_time.is_none() {
					log_str("+___125");
					pause.pause_time
				} else {
					log_str("+___126");
					Utc::now()
				}
			} else {
				log_str("+___127");
				Utc::now()
			}
		}));
		let pauses = &self.pauses;

		let mut pauses_time_duration = Duration::zero();
		let full_pauses = pauses.iter().filter(|it| it.resume_time.is_some());
		for pause in full_pauses {
			pauses_time_duration = pauses_time_duration + (pause.resume_time.unwrap() - pause.pause_time);
		}

		let duration = local_end_time.naive_local() - local_start_time.naive_local() - pauses_time_duration;

		// pauses = {pauses:#?},
		// full_pauses = {pauses:#?},
		format!("self_start_time = {self_start_time:#?},
start_time = {start_time},
local_start_time = {local_start_time},
self.end_time = {self_end_time:#?},
local_end_time = {local_end_time},

pauses_time_duration = {pauses_time_duration},
duration = {duration},
		")
	}
}