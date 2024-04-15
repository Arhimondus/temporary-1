use chrono::{DateTime, Utc, Duration, Local};
use js_sys::Date;
use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{utils::string_time_to_int, state::State, components::StampType};

use super::{OperationResult, Pause};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OperationPassStatus {
	Open = 0,
	Canceled = 1,
	Zero = 2,
	Partial = 3,
	Full = 4,
}

impl From<OperationPassStatus> for StampType {
	fn from(value: OperationPassStatus) -> StampType {
		match value {
			OperationPassStatus::Open => StampType::Full,
			OperationPassStatus::Canceled => StampType::Zero,
			OperationPassStatus::Zero => StampType::Zero,
			OperationPassStatus::Partial => StampType::Partial,
			OperationPassStatus::Full => StampType::Full,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Operation {
	id: u32,
	order_id: u32,
	user_id: u32,
	task_id: u32,
	name: String,
	duration: Option<u32>, // TODO!! При пограничный ситуация в полуночи приходит отрицательный duration
	floating: bool,

	description: Option<String>,
	// details: Option<String>,
	unit: String,
	count: String,
	plan_count: f32,
	pass_count: Option<f32>,
	accept_count: Option<f32>,
	single_price: f32,

	pass_comment: Option<String>,
	accept_comment: Option<String>,

	closed: bool,
	
	// deleted: bool,
	// single_price: f64,
	// start_time: Option<DateTime<Utc>>,
	// end_time: Option<DateTime<Utc>>,
	pass_start_time: Option<DateTime<Utc>>,
	pass_end_time: Option<DateTime<Utc>>,

	delta: u32,
	total_delta: u32,

	pass_status: OperationPassStatus,

	pass_pauses: Vec<Pause>,
}

impl Timeable for OperationResult {
	fn start_time(&self) -> DateTime<Utc> {
		self.start_time.unwrap()
	}

	fn end_time(&self) -> DateTime<Utc> {
		self.end_time.unwrap_or(chrono::Utc::now())
	}

	fn duration(&self) -> Duration {
		Timeable::end_time(self) - Timeable::start_time(self)
	}
}

impl Timeable for Operation {
	fn start_time(&self) -> DateTime<Utc> {
		self.pass_start_time.unwrap()
	}

	fn end_time(&self) -> DateTime<Utc> {
		self.pass_end_time.unwrap_or(chrono::Utc::now())
	}

	fn duration(&self) -> Duration {
		Timeable::end_time(self) - Timeable::start_time(self)
	}
}

pub trait Timeable {
	fn start_time(&self) -> DateTime<Utc>;
	fn end_time(&self) -> DateTime<Utc>;
	fn duration(&self) -> Duration;
	fn duration_time(&self) -> (i64, i64) {
		let duration = Timeable::end_time(self) - Timeable::start_time(self);
		let minutes = duration.num_minutes();		
		let seconds = duration.num_seconds() - minutes *  60;
		(minutes, seconds)		
	}
	fn duration_minutes(&self) -> i64 {
		let dt = self.duration_time();
		dt.0
	}
}

pub trait TimeableOperation : Timeable {
	fn pure_duration(&self) -> Duration;
	fn pauses_duration(&self) -> Duration;
}

pub trait BeautyDuration {
	fn mins_and_secs(&self) -> (i64, i64);
}

impl BeautyDuration for Duration {
	fn mins_and_secs(&self) -> (i64, i64) {
		todo!()
	}
}

pub trait BeautyDateTime {
	fn local_time(self) -> DateTime<Local>;
}

impl BeautyDateTime for DateTime<Utc> {
	fn local_time(self) -> DateTime<Local> {
		let local_time: DateTime<Local> = DateTime::from(self);
		local_time
	}
}

impl From<&Operation> for OperationResult  {
	// Только для создания при запуске задачи в работу?
	fn from(value: &Operation) -> Self {
		OperationResult {
			operation_id: value.id,
			count: None, //value.plan_count,
			comment: None,
			start_time: None,
			end_time: None, 
			delta: 0, 
			total_delta: 0, 
			status: super::operation_result::OperationResultStatus::Open,
			pauses: vec![],
		}
	}
}

impl OperationResult {
	pub fn is_current(&self, state: &State) -> bool {
		let Some(active_task) = state.active_task.as_ref() else { return false };
		let Some(operation_id) = active_task.current_operation_id else { return false };
		self.operation_id == operation_id
	}

	pub fn pauses_duration(&self) -> Duration {
		self.pauses.iter()
			.map(|p| Timeable::duration(p))
			.try_fold(Duration::zero(), |acc, d| acc.checked_add(&d)).unwrap()
	}
}

impl Operation {
	pub fn pass(&self, state: &mut State) {
		let Some(active_task) = state.active_task.as_mut() else { panic!() };
		active_task.operation_results.get_mut(&self.id).unwrap().pass();
	}

	pub fn upload(&self, state: &mut State) {
		let Some(active_task) = state.active_task.as_mut() else { panic!() };
		active_task.operation_results.get_mut(&self.id).unwrap().upload();
	}

	pub fn upload_successfull(&self, state: &mut State) {
		let Some(active_task) = state.active_task.as_mut() else { panic!() };
		active_task.operation_results.get_mut(&self.id).unwrap().upload_successfull();
	}
	
	pub fn upload_failure(&self, state: &mut State) {
		let Some(active_task) = state.active_task.as_mut() else { panic!() };
		active_task.operation_results.get_mut(&self.id).unwrap().upload_failure();
	}

	pub fn is_current(&self, state: &State) -> bool {
		let Some(active_task) = state.active_task.as_ref() else { return false };
		let Some(operation_id) = active_task.current_operation_id else { return false };
		self.id == operation_id
	}

	pub fn get_operation_result<'a>(&self, state: &'a mut State) -> Option<&'a mut OperationResult> {
		let Some(active_task) = state.active_task.as_mut() else { return None };
		let Some(current_operation_id) = active_task.current_operation_id else { return None };
		active_task.operation_results.get_mut(&current_operation_id)
	}

	// pub fn end_time(&self) -> String {
	// 	match &self.end_time {
	// 		Some(a) => format!("{}", a.format("%H:%M")),
	// 		None => "".to_string(),
	// 	}
	// }

	pub fn time(&self, or: &OperationResult) -> String {
		"".into()
	}

	// pub fn active_start_time(&self, active_operation: ActiveOperation) -> u64 {
	// 	let (hours, minutes) = string_time_to_int(&self.start_time.clone().unwrap());
	// 	active_operation.pause_millis
	// }

	// pub fn active_end_time(&self, active_operation: ActiveOperation) -> u64 {
	// 	let (hours, minutes) = string_time_to_int(&self.end_time.clone().unwrap());
	// 	active_operation.pause_millis
	// }

	// pub fn potential_time(&self) -> String {
	// 	let local_start_time: DateTime<Local> = DateTime::from(self.start_time);
	// 	let local_end_time: DateTime<Local> = DateTime::from(chrono::Utc::now());
	// 	let (minutes, seconds) = self.duration_time();
	// 	format!("{} - {} (чистое время - {} минут, {} секунд, {} пауз - {})",
	// 		local_start_time.format("%H:%M"),
	// 		local_end_time.format("%H:%M"),
	// 		minutes,
	// 		seconds,
	// 		self.pauses.len(),
	// 		self.pauses()
	// 	)
	// }
}