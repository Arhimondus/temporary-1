use chrono::{DateTime, Utc, Local, Duration};
use pub_this::pub_this;
use serde::{Deserialize, Serialize};

use super::Timeable;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[pub_this]
pub struct Pause {
	pause_time: DateTime<Utc>,
	resume_time: Option<DateTime<Utc>>,
	comment: Option<String>,
}

impl Timeable for Pause {
	fn start_time(&self) -> DateTime<Utc> {
		self.pause_time
	}

	fn end_time(&self) -> DateTime<Utc> {
		self.resume_time.unwrap_or(chrono::Utc::now())
	}

	fn duration(&self) -> Duration {
		Timeable::end_time(self) - Timeable::start_time(self)
	}
}

impl Pause {
	pub fn duration(&self) -> Option<String> {
		if let Some(end_time) = self.resume_time {
			let duration = end_time - self.pause_time;
			// Some(format!("{}:{}", duration.num_minutes(), duration.num_seconds()))
			Some(format!("{} мин", duration.num_minutes()))
		} else {
			None
		}
	}

	pub fn time(&self) -> String {
		let local_start_time: DateTime<Local> = DateTime::from(self.pause_time);
		format!("{} - {}", local_start_time.format("%H:%M:%S"), if let Some(end_time) = self.resume_time {
			let local_end_time: DateTime<Local> = DateTime::from(end_time);
			format!("{} | {}", local_end_time.format("%H:%M:%S"), self.duration().unwrap())
		} else { "".into() })						
	}
}