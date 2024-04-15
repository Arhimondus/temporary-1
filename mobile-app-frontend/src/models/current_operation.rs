use pub_this::pub_this;
use yewdux::store::Store;
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime, Duration, Local};
use crate::models::BeautyDateTime;

use super::{Pause, Operation, OperationResult, TimeableOperation, Timeable};

#[derive(Default, Clone, PartialEq, Store, Serialize, Deserialize)]
#[pub_this]
pub struct CurrentOperation {
	operation_id: u32,
	start_time: DateTime<Utc>,
	pauses: Vec<Pause>,
	delta: u32,
	total_delta: u32,
}

impl Timeable for CurrentOperation {
	fn start_time(&self) -> DateTime<Utc> {
		self.start_time
	}

	fn end_time(&self) -> DateTime<Utc> {
		chrono::Utc::now()
	}

	fn duration(&self) -> Duration {
		Timeable::end_time(self) - Timeable::start_time(self)
	}
}

impl TimeableOperation for CurrentOperation {
	fn pure_duration(&self) -> Duration {
		Timeable::end_time(self) - Timeable::start_time(self) - TimeableOperation::pauses_duration(self)
	}

	fn pauses_duration(&self) -> Duration {
		self.pauses.iter()
			.map(|p| Timeable::duration(p))
			.try_fold(Duration::zero(), |acc, d| acc.checked_add(&d)).unwrap()
	}
}

impl CurrentOperation {
	pub fn pauses_time(&self) -> (i64, i64) {

		let duration = self.pauses_duration();

		let minutes = duration.num_minutes();		
		let seconds = duration.num_seconds() - minutes *  60;
		(minutes, seconds)
	}

	pub fn pauses(&self) -> String {
		let (minutes, seconds) = self.pauses_time();
		format!("{} минут, {} секунд", minutes, seconds)
	}

	pub fn duration(&self) -> String {
		let (minutes, seconds) = self.duration_time();
		format!("{:0>2}:{:0>2}", minutes, seconds)		
	}

	pub fn start_time(&self) -> String {
		let local_time: DateTime<Local> = DateTime::from(self.start_time);
		local_time.format("%H:%M").to_string()
	}

	pub fn potential_time(&self) -> String {
		let local_start_time: DateTime<Local> = self.start_time.local_time();
		let local_end_time: DateTime<Local> = chrono::Local::now();
		let (minutes, seconds) = self.duration_time();
		format!("{} - {} (чистое время - {} минут, {} секунд, {} пауз - {})",
			local_start_time.format("%H:%M"),
			local_end_time.format("%H:%M"),
			minutes,
			seconds,
			self.pauses.len(),
			self.pauses()
		)
	}
}