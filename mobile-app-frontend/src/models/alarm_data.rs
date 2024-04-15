use std::{fmt::{Display, self}, ops::RangeBounds};

use chrono::{DateTime, Utc, Local, Timelike, TimeZone, Datelike, NaiveDate, NaiveTime, NaiveDateTime};
use js_sys::Boolean;
use pub_this::pub_this;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActionType {
	StartOperation = 0,
	EndOperation = 1,
	EndOperationRest = 2,
	EndOperationLast = 3,
	NoOperation = 4,

	StartTask = 5,
	EndTask = 6,

	RemovePermanentNotification = 7,
	RemoveAllNotifications = 8,
	FinishAutoOperation = 9,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct AlarmAction {
	js_action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct AlarmNotification {
	title: String,
	text: String,
	permanent: bool,
	open_url: String,
	channel: String,
	group: Option<String>,
	silent: bool,
	click_action: Option<AlarmAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct PlaySound {
	text_speach: Option<String>,
	sound_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct RawAction {
	r#type: ActionType,
	task_id: Option<u32>,
	operation_id: Option<u32>,
	custom_data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Time {
	hours: u8,
	minutes: u8,
	seconds: u8,
}

impl Time {
	fn time(self) -> DateTime<Utc> {
		let now = Utc::now();
		Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), self.hours.into(), self.minutes.into(), self.seconds.into()).unwrap()
	}
	fn naive_time(self) -> NaiveTime {
		let now = Local::now();
		Local.with_ymd_and_hms(now.year(), now.month(), now.day(), self.hours.into(), self.minutes.into(), self.seconds.into()).unwrap().time()
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Alarm {
	_type: ActionType,
	_operation_id: Option<u32>,
	time: Time,
	play_sound: Option<String>,
	speech_text: Option<String>,
	alarm_action: Option<AlarmAction>,
	notification: Option<AlarmNotification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct AlarmItem {
	operation_id: u32,
	name: String,
	start_time: Time,
	end_time: Time,
	after_rest: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct AlarmItems {
	items: Vec<AlarmItem>,
	task_id: u32,
}

impl AlarmItems {
	fn new(task_id: u32) -> Self {
		Self { items: vec![], task_id }
	}
}

const OPERATION_CHANNEL: &str = "operationchannel";

macro_rules! al {
	(StartOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::StartOperation,
				_operation_id: Some($operation_id),
				time: $time,
				notification: Some(AlarmNotification {
					title: $title.into(),
					text: $description.into(),
					permanent: true,
					open_url: "/work-task".into(),
					channel: OPERATION_CHANNEL.into(),
					group: None,
					silent: false,
					click_action: Some(AlarmAction {
						js_action: "".into()
					}),
				}),
				play_sound: Some("short_sound.mp3".into()),
				speech_text: Some(format!("Начало операции {}", $description)),
				alarm_action:Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::StartOperation,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
	(EndOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::EndOperation,
				_operation_id: Some($operation_id),
				time: $time,
				notification: Some(AlarmNotification {
					title: $title.into(),
					text: $description.into(),
					permanent: false,
					open_url: format!("/app/index.html#/finish-auto-operation/{}/{}", $task_id, $operation_id),
					channel: OPERATION_CHANNEL.into(),
					group: Some("finished-operation".into()),
					silent: false,
					click_action: Some(AlarmAction {
						js_action: serde_json::to_string(&RawAction {
							operation_id: Some($operation_id),
							task_id: Some($task_id),
							custom_data: Some(format!("/app/index.html#/finish-auto-operation/{}/{}", $task_id, $operation_id).into()),
							r#type: ActionType::FinishAutoOperation,
						}).unwrap().replace("\"", "\\\""),
					}),
				}),
				play_sound: None,
				speech_text: None,
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::EndOperation,
					}).unwrap().replace("\"", "\\\"")
				}),
			}
		}
	};
	(EndOperationRest, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::EndOperationRest,
				_operation_id: Some($operation_id),
				time: $time,
				notification: Some(AlarmNotification {
					title: $title.into(),
					text: $description.into(),
					permanent: false,
					open_url: format!("/finish-auto-operation/{}/{}", $task_id, $operation_id),
					channel: OPERATION_CHANNEL.into(),
					group: Some("finished-operation".into()),
					silent: false,
					click_action: Some(AlarmAction {
						js_action: "".into()
					}),
				}),
				play_sound: Some("long_sound.mp3".into()),
				speech_text: Some(format!("Завершение операции {}, перерыв", $description)),
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::EndOperationRest,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
	(EndOperationLast, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::EndOperationLast,
				_operation_id: Some($operation_id),
				time: $time,
				notification: Some(AlarmNotification {
					title: $title.into(),
					text: $description.into(),
					permanent: false,
					open_url: format!("/finish-auto-operation/{}/{}", $task_id, $operation_id),
					channel: OPERATION_CHANNEL.into(),
					group: Some("finished-operation".into()),
					silent: false,
					click_action: Some(AlarmAction {
						js_action: "".into()
					}),
				}),
				play_sound: Some("long_sound.mp3".into()),
				speech_text: Some(format!("Завершение операции {}, выберите следующую", $description)),
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::EndOperationLast,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
	(NoOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::RemovePermanentNotification,
				_operation_id: Some($operation_id),
				time: $time,
				notification: Some(AlarmNotification {
					title: $title.into(),
					text: $description.into(),
					permanent: false,
					open_url: "/work-task".into(),
					channel: OPERATION_CHANNEL.into(),
					group: None,
					silent: false,
					click_action: Some(AlarmAction {
						js_action: "".into()
					}),
				}),
				play_sound: None,
				speech_text: None,
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::RemovePermanentNotification,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
	(StartTask, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::StartTask,
				_operation_id: Some($operation_id),
				time: $time,
				notification: None,
				play_sound: None,
				speech_text: None,
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::StartTask,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
	(EndTask, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			Alarm {
				_type: ActionType::EndTask,
				_operation_id: Some($operation_id),
				time: $time,
				notification: None,
				play_sound:Some("long_sound.mp3".into()),
				speech_text: Some("Задача полностью выполнена"),
				alarm_action: Some(AlarmAction {
					js_action: serde_json::to_string(&RawAction {
						operation_id: Some($operation_id),
						task_id: Some($task_id),
						custom_data: None,
						r#type: ActionType::EndTask,
					}).unwrap().replace("\"", "\\\""),
				}),
			}
		}
	};
}

macro_rules! ad {
	(StartOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(StartOperation, operation $operation_id, task $task_id, $time.clone(), $title, $description)
		}	
	};
	(EndOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(EndOperation, operation $operation_id, task $task_id, $time.clone(), $title, $description)
		}	
	};
	(EndOperationRest, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(EndOperationRest, operation $operation_id, task $task_id, $time.clone(), $title, $description)
		}	
	};
	(EndOperationLast, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(EndOperationLast, operation $operation_id, task $task_id, $time.clone(), $title, $description)
		}	
	};
	(NoOperation, operation $operation_id: expr, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(NoOperation, operation $operation_id, task $task_id, $time.clone(), $title, $description)
		}	
	};
	(StartTask, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(StartTask, task $task_id, $time.unwrap(), $title, $description)
		}
	};
	(EndTask, task $task_id: expr, $time: expr, $title: expr, $description: expr) => {
		{
			al!(EndTask, task $task_id, $time.unwrap(), $title, $description)
		}
	};
}

impl Into<Vec<Alarm>> for AlarmItems {
	fn into(self) -> Vec<Alarm> {
		let mut alarm_datas: Vec<Alarm> = vec![];
		let task_id = self.task_id;
		for i in 0..self.items.len() {
			let op = self.items.get(i).unwrap();
			let next = self.items.get(i + 1);
			let operation_id = op.operation_id;
			let start_time = &op.start_time;
			let end_time = &op.end_time;

			alarm_datas.push(ad!(StartOperation, operation operation_id, task task_id, start_time, "Текущая операция", op.name.clone()));

			if next.is_none() {
				alarm_datas.push(ad!(NoOperation, operation op.operation_id, task task_id, end_time, "Нет активной операции", op.name.clone()));
				alarm_datas.push(ad!(EndOperationLast, operation op.operation_id, task task_id, end_time, "Завершилась операция", op.name.clone()));
			} else if let Some(next) = next {
				alarm_datas.push(ad!(EndOperation, operation op.operation_id, task task_id, end_time, "Завершение операции", op.name.clone()));
			}
		}
		alarm_datas
	}
}

impl AlarmItems {
	pub fn get_current_operation(&self) -> Option<(u32, DateTime<Utc>)> {
		let now = Local::now().time();
		if let Some(alarm_item) = self.items.iter().find(|it| {
			let start_time = it.start_time.clone().naive_time();
			let end_time = it.end_time.clone().naive_time();
			(start_time..end_time).contains(&now)
		}) {
			Some((alarm_item.operation_id, alarm_item.start_time.clone().time()))
		} else {
			None
		}
	}
}

impl fmt::Display for Alarm {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:0>2}:{:0>2}.{:0>2} action", self.time.hours, self.time.minutes, self.time.seconds)
		// if let Some(action) = self.alarm_action.as_ref() {
		// 	write!(f, "{:0>2}:{:0>2}.{:0>2} {} {}", self.time.hours, self.time.minutes, self.time.seconds, match action.r#type {
		// 			ActionType::StartOperation => "start operation",
		// 			ActionType::EndOperation => "end operation",
		// 			ActionType::EndOperationRest => "end operation & rest",
		// 			ActionType::EndOperationLast => "end last operation",
		// 			ActionType::NoOperation => "no operation",
		// 			ActionType::StartTask => "start task",
		// 			ActionType::EndTask => "end task",
		// 			ActionType::RemovePermanentNotification => "remove permanent notification",
		// 			ActionType::RemoveAllNotifications => "remova all notifications",
		// 		}, {
		// 		"@".into()
		// 	})
		// } else {
		// 	write!(f, "{:0>2}:{:0>2}.{:0>2} no action", self.time.hours, self.time.minutes, self.time.seconds)
		// }
	}
}

impl From<DateTime<Utc>> for Time {
	fn from(value: DateTime<Utc>) -> Self {
		let local_time: DateTime<Local> = DateTime::from(value);
		Time {
			hours: local_time.hour() as u8,
			minutes: local_time.minute() as u8,
			seconds: local_time.second() as u8,
		}
	}
}