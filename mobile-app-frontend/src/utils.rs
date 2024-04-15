use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Add;

use chrono::{Duration, TimeZone, Datelike, DateTime, Utc, Local};
use js_sys::{Boolean, eval};
use log::debug;
use log_derive::logfn;
use pub_this::pub_this;
// use fermi::{Atom, AtomRef};
use reqwest::{Error, Body};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{Html, html};
use one_of::one_of;
use one_of::case;
use gloo::storage::{LocalStorage, Storage};


use crate::models::*;
use crate::state::AppStateActions;
use crate::state::State;

// static PLANNERKEY: &str = dotenv!("PLANNER_KEY");
// static BACKEND_URL: &str = "http://195.112.108.82:8090/api";
static BACKEND_URL: &str = dotenv!("BACKEND_URL");
// static BACKEND_URL: &str = "http://192.168.1.209:9091";
// static BACKEND_URL: &str = "http://217.15.197.70:9090";

#[derive(Clone, Deserialize, PartialEq)]
#[pub_this]
pub struct ApiError {
	error: String,
	global: bool,
	stack: Option<String>,
	status: u16,
}

pub fn stamp_from_operation(operation: &Operation) -> Html {
	stamp_from_pass_and_accept_count(operation.pass_count, operation.accept_count)
}

pub fn stamp_from_operation_and_operation_result(operation: &Operation, operation_result: &OperationResult) -> Html {
	stamp_from_pass_and_accept_count(operation_result.count, operation.accept_count)
}

pub fn stamp_from_pass_and_accept_count(pass_count: Option<f32>, accept_count: Option<f32>) -> Html {
	if let Some(pass_count) = pass_count {	
		if let Some(accept_count) = accept_count {
			if accept_count == pass_count {
				return html! { <div class="panel-block has-background-success has-text-white is-flex is-justify-content-space-between">
					{"Полностью"}
				</div> }
			} else if accept_count > 0.0 {
				return html! { <div class="panel-block has-background-warning has-text-white is-flex is-justify-content-space-between">
					{"Частично"}
				</div> }
			} else {
				return html! { <div class="panel-block has-background-danger has-text-white is-flex is-justify-content-space-between">
					{"Отклонено"}
				</div> }
			}
		}
	}
	html!{ <></> }
}

fn operation_color(operation: &Operation, operation_result: &OperationResult) -> String {
	let operation_result = operation_result.count.unwrap_or_else(|| operation.pass_count.unwrap());
	let op_color = if operation_result == operation.plan_count {
		"operation-status-full"
	} else if operation_result == 0. {
		"operation-status-zero"
	} else {
		"operation-status-partial"
	};
	op_color.into()
}

fn operation_color_finished(operation: &Operation) -> String {
	let Some(pass_count) = operation.pass_count else {
		return "operation-status-zero".to_string();
	};
	let op_color = if pass_count == operation.plan_count {
		"operation-status-full"
	} else if pass_count == 0. {
		"operation-status-zero"
	} else {
		"operation-status-partial"
	};
	op_color.into()
}

pub fn operation_background(operation: &Operation, operation_result: &Option<OperationResult>, current_operation: bool) -> String {
	if let Some(operation_result) = operation_result {
		match operation_result.status {
			OperationResultStatus::Open => "status-open panel-block has-background-white has-text-black is-flex is-justify-content-space-between".into(),
			OperationResultStatus::Pass => {
				if current_operation {
					// Операция была уже "пройдена" и является текущей
					"status-pass-is-current panel-block has-background-info has-text-white is-flex is-justify-content-space-between".into()
				} else {
					// Операция была уже "пройдена"
					if operation_result.count.is_some() {
						format!("status-pass panel-block {} has-text-white is-flex is-justify-content-space-between", operation_color(&operation, &operation_result)).into()
					} else {
						"status-pass panel-block has-background-primary has-text-white is-flex is-justify-content-space-between".into()
					}
				}
			},
			OperationResultStatus::Done(_) => {
				format!("status-done panel-block {} has-text-white is-flex is-justify-content-space-between", operation_color(&operation, &operation_result)).into()
			},
		}
	} else {
		format!("status-done panel-block {} has-text-white is-flex is-justify-content-space-between", operation_color_finished(&operation)).into()
	}
}

pub fn operation_done_icon(operation_result: &OperationResult) -> Html {
	if let OperationResultStatus::Done(operation_done_status) = &operation_result.status {
		match operation_done_status {
			OperationDoneStatus::Uploading => html! { <img style="width: 32px;" src="/static/loader.png"/> }, // <i class="fa fa-cloud-upload mr-2"></i>
			OperationDoneStatus::Successfull => html! { <i class="fa fa-check mr-2"></i> },
			OperationDoneStatus::Failure => html! { <i class="fa fa-exclamation mr-2"></i> },
		}
	} else {
		html! { <></> }
	}
}

pub fn string_time_to_int(time: &String) -> (u32, u32) {
	let splitted = time.split(":").collect::<Vec<_>>();
	let hours = splitted[0].parse::<u32>().unwrap();
	let minutes = splitted[1].parse::<u32>().unwrap();
	return (hours, minutes);
}

pub fn short_time(time: &Option<DateTime<Utc>>) -> String {
	if let Some(time) = time {
		format!("{}", time.format("%H:%M"))
	} else {
		"?".into()
	}
}

pub fn short_time2(time: &DateTime<Utc>) -> String {
	let local_time: DateTime<Local> = DateTime::from(time.clone());
	format!("{}", local_time.format("%H:%M"))
}

#[cfg(test)]
mod tests {
    use crate::models::{OperationResult, Operation, OperationResultStatus, Alarm, ActionType, Task};

    use super::get_alarm_items;

	macro_rules! f {
		($operation_id: tt, task $task_id: tt, $operation_name: tt, $duration: tt min, $plan_count: tt/$pass_count: tt/$accept_count: tt, $single_price: tt => $status: tt, $count: tt,  $start_time: tt - $end_time: tt, $delta: tt/$total_delta: tt) => {
			{
				let user_id = 5;
				(&Operation { id:1,	user_id: user_id, task_id: $task_id, name: $operation_name.into(), duration: $duration, floating: true, description: None, unit: "мм.".into(), count: $plan_count.to_string().into(),
					plan_count: $plan_count as f32, pass_count: Some($pass_count as f32), accept_count: Some($accept_count as f32), single_price: $single_price as f32, closed: false, delta: 0, total_delta: 0,
				}, &OperationResult { operation_id: $operation_id, count: $count, comment: Some("asd".into()), start_time: None, end_time: None,
					delta: 0, total_delta: 0, status: OperationResultStatus::$status, pauses: vec![],
				})	
			}
		};
	}

	macro_rules! g {
		(operation $operation_id: tt, task $task_id: tt, $operation_name: tt, $duration: tt min, $plan_count: tt/$pass_count: tt/$accept_count: tt, $single_price: tt => $status: tt, $count: expr, $start_time: tt - $end_time: tt, $delta: tt/$total_delta: tt) => {
			{
				let user_id = 5;
				(&Operation { id:1,	user_id: user_id, task_id: $task_id, name: $operation_name.into(), duration: $duration, floating: false, description: None, unit: "мм.".into(), count: $plan_count.to_string().into(),
					plan_count: $plan_count as f32, pass_count: Some($pass_count as f32), accept_count: Some($accept_count as f32), single_price: $single_price as f32, closed: false, delta: 0, total_delta: 0,
				}, &OperationResult { operation_id: $operation_id, count: $count, comment: Some("asd".into()), start_time: None, end_time: None,
					delta: 0, total_delta: 0, status: OperationResultStatus::$status, pauses: vec![],
				})	
			}
		};
	}

	#[test]
	fn dobr() {
		let data = r#"[{"id":266,"date":"2023-07-07T21:00:00.000Z","sector_id":1,"start_time":"2023-07-12T18:43:14.495Z","end_time":"2023-07-12T18:54:14.495Z","name":"Монтаж лоджии","description":"Смонтируйте лоджию!","floating":false,"closed":false,"deleted":false,"plan_id":203,"sector":"Фасад 1","address":"Объект 3","type":0,"operations":[{"id":1145,"user_id":41,"task_id":266,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:43:14.495Z","end_time":"2023-07-12T18:44:14.495Z","floating":false,"unit":"м²","order_id":0,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1146,"user_id":41,"task_id":266,"name":"Сборка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:44:14.495Z","end_time":"2023-07-12T18:45:14.495Z","floating":false,"unit":"м²","order_id":1,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1147,"user_id":41,"task_id":266,"name":"Монтаж","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:45:14.495Z","end_time":"2023-07-12T18:46:14.495Z","floating":false,"unit":"м²","order_id":2,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1148,"user_id":41,"task_id":266,"name":"Проверка","description":null,"details":null,"unit_id":5,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:48:14.495Z","end_time":"2023-07-12T18:50:14.495Z","floating":false,"unit":"шт","order_id":3,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":2},{"id":1149,"user_id":41,"task_id":266,"name":"Сверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:50:14.495Z","end_time":"2023-07-12T18:51:14.495Z","floating":false,"unit":"м²","order_id":4,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1150,"user_id":41,"task_id":266,"name":"Доверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:51:14.495Z","end_time":"2023-07-12T18:52:14.495Z","floating":false,"unit":"м²","order_id":5,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1151,"user_id":41,"task_id":266,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:52:14.495Z","end_time":"2023-07-12T18:53:14.495Z","floating":false,"unit":"м²","order_id":6,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1152,"user_id":41,"task_id":266,"name":"Сдача работы","description":null,"details":null,"unit_id":5,"count":"1.02","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":150.01,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T18:53:14.495Z","end_time":"2023-07-12T18:54:14.495Z","floating":false,"unit":"шт","order_id":7,"status":0,"plan_salary":153.0102,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":15.30102,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":1.02,"duration":1}],"plan_salary":1027.6602,"pass_salary":0,"accept_salary":0},{"id":274,"date":"2023-07-10T21:00:00.000Z","sector_id":1,"start_time":"2023-07-12T16:31:14.502Z","end_time":"2023-07-12T16:42:14.502Z","name":"Монтаж лоджии","description":"Смонтируйте лоджию!","floating":false,"closed":false,"deleted":false,"plan_id":211,"sector":"Фасад 1","address":"Объект 3","type":2,"operations":[{"id":1178,"user_id":41,"task_id":274,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:31:14.502Z","end_time":"2023-07-12T16:32:14.502Z","floating":true,"unit":"м²","order_id":0,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1179,"user_id":41,"task_id":274,"name":"Сборка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:32:14.502Z","end_time":"2023-07-12T16:33:14.502Z","floating":false,"unit":"м²","order_id":1,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1180,"user_id":41,"task_id":274,"name":"Монтаж","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:33:14.502Z","end_time":"2023-07-12T16:34:14.502Z","floating":false,"unit":"м²","order_id":2,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1181,"user_id":41,"task_id":274,"name":"Проверка","description":null,"details":null,"unit_id":5,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:36:14.502Z","end_time":"2023-07-12T16:38:14.502Z","floating":false,"unit":"шт","order_id":3,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":2},{"id":1182,"user_id":41,"task_id":274,"name":"Сверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:38:14.502Z","end_time":"2023-07-12T16:39:14.502Z","floating":true,"unit":"м²","order_id":4,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1183,"user_id":41,"task_id":274,"name":"Доверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:39:14.502Z","end_time":"2023-07-12T16:40:14.502Z","floating":true,"unit":"м²","order_id":5,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1184,"user_id":41,"task_id":274,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:40:14.502Z","end_time":"2023-07-12T16:41:14.502Z","floating":true,"unit":"м²","order_id":6,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1185,"user_id":41,"task_id":274,"name":"Сдача работы","description":null,"details":null,"unit_id":5,"count":"1.02","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":150.01,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T16:41:14.502Z","end_time":"2023-07-12T16:42:14.502Z","floating":false,"unit":"шт","order_id":7,"status":0,"plan_salary":153.0102,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":15.30102,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":1.02,"duration":1}],"plan_salary":1027.6602,"pass_salary":0,"accept_salary":0},{"id":272,"date":"2023-07-10T21:00:00.000Z","sector_id":1,"start_time":"2023-07-12T12:59:14.500Z","end_time":"2023-07-12T13:10:14.500Z","name":"Монтаж лоджии","description":"Смонтируйте лоджию!","floating":false,"closed":false,"deleted":false,"plan_id":209,"sector":"Фасад 1","address":"Объект 3","type":0,"operations":[{"id":1172,"user_id":41,"task_id":272,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T12:59:14.500Z","end_time":"2023-07-12T13:00:14.500Z","floating":false,"unit":"м²","order_id":0,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1173,"user_id":41,"task_id":272,"name":"Сборка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T13:00:14.500Z","end_time":"2023-07-12T13:01:14.500Z","floating":false,"unit":"м²","order_id":1,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1174,"user_id":41,"task_id":272,"name":"Монтаж","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T13:01:14.500Z","end_time":"2023-07-12T13:02:14.500Z","floating":false,"unit":"м²","order_id":2,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1}],"plan_salary":374.84999999999997,"pass_salary":0,"accept_salary":0},{"id":270,"date":"2023-07-09T21:00:00.000Z","sector_id":1,"start_time":"2023-07-12T00:18:14.497Z","end_time":"2023-07-12T00:29:14.497Z","name":"Монтаж лоджии","description":"Смонтируйте лоджию!","floating":false,"closed":false,"deleted":false,"plan_id":207,"sector":"Фасад 1","address":"Объект 3","type":2,"operations":[{"id":1164,"user_id":41,"task_id":270,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":"15:42:00","pass_end_time":"18:09:00","pass_count":5.1,"pass_comment":null,"pass_pauses":"[{\"comment\":null,\"pause_time\":\"15:42\",\"resume_time\":\"15:42\"},{\"comment\":null,\"pause_time\":\"15:42\",\"resume_time\":\"15:42\"},{\"comment\":null,\"pause_time\":\"15:42\",\"resume_time\":\"15:42\"},{\"comment\":null,\"pause_time\":\"15:42\",\"resume_time\":\"15:42\"},{\"comment\":null,\"pause_time\":\"15:42\",\"resume_time\":\"15:42\"}]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":"2023-07-12T15:09:31.180Z","delta":0,"total_delta":0,"start_time":"2023-07-12T00:18:14.497Z","end_time":"2023-07-12T00:19:14.497Z","floating":true,"unit":"м²","order_id":0,"status":1,"plan_salary":124.94999999999999,"pass_salary":124.94999999999999,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":12.495,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1165,"user_id":41,"task_id":270,"name":"Сборка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:19:14.497Z","end_time":"2023-07-12T00:20:14.497Z","floating":true,"unit":"м²","order_id":1,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1},{"id":1166,"user_id":41,"task_id":270,"name":"Монтаж","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:20:14.497Z","end_time":"2023-07-12T00:21:14.497Z","floating":false,"unit":"м²","order_id":2,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1167,"user_id":41,"task_id":270,"name":"Проверка","description":null,"details":null,"unit_id":5,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:23:14.497Z","end_time":"2023-07-12T00:25:14.497Z","floating":false,"unit":"шт","order_id":3,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":2},{"id":1168,"user_id":41,"task_id":270,"name":"Сверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:25:14.497Z","end_time":"2023-07-12T00:26:14.497Z","floating":false,"unit":"м²","order_id":4,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1}],"plan_salary":624.75,"pass_salary":124.94999999999999,"accept_salary":0},{"id":265,"date":"2023-07-04T21:00:00.000Z","sector_id":1,"start_time":"2023-07-11T23:59:14.492Z","end_time":"2023-07-12T00:10:14.492Z","name":"Монтаж лоджии","description":"Смонтируйте лоджию!","floating":false,"closed":false,"deleted":false,"plan_id":202,"sector":"Фасад 1","address":"Объект 3","type":0,"operations":[{"id":1137,"user_id":41,"task_id":265,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-11T23:59:14.492Z","end_time":"2023-07-12T00:00:14.492Z","floating":false,"unit":"м²","order_id":0,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1138,"user_id":41,"task_id":265,"name":"Сборка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:00:14.492Z","end_time":"2023-07-12T00:01:14.492Z","floating":false,"unit":"м²","order_id":1,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1139,"user_id":41,"task_id":265,"name":"Монтаж","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:01:14.492Z","end_time":"2023-07-12T00:02:14.492Z","floating":false,"unit":"м²","order_id":2,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1140,"user_id":41,"task_id":265,"name":"Проверка","description":null,"details":null,"unit_id":5,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:04:14.492Z","end_time":"2023-07-12T00:06:14.492Z","floating":false,"unit":"шт","order_id":3,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":2},{"id":1141,"user_id":41,"task_id":265,"name":"Сверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:06:14.492Z","end_time":"2023-07-12T00:07:14.492Z","floating":false,"unit":"м²","order_id":4,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1142,"user_id":41,"task_id":265,"name":"Доверка","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:07:14.492Z","end_time":"2023-07-12T00:08:14.492Z","floating":false,"unit":"м²","order_id":5,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1143,"user_id":41,"task_id":265,"name":"Переодевание","description":null,"details":null,"unit_id":1,"count":"5.1","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":24.5,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:08:14.492Z","end_time":"2023-07-12T00:09:14.492Z","floating":false,"unit":"м²","order_id":6,"status":0,"plan_salary":124.94999999999999,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":12.495,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":5.1,"duration":1},{"id":1144,"user_id":41,"task_id":265,"name":"Сдача работы","description":null,"details":null,"unit_id":5,"count":"1.02","closed":false,"pass_start_time":null,"pass_end_time":null,"pass_count":null,"pass_comment":null,"pass_pauses":"[]","pass_deltas":null,"accept_time":null,"accept_count":null,"accept_comment":null,"accept_user_id":null,"deleted":false,"single_price":150.01,"pass_time":null,"delta":0,"total_delta":0,"start_time":"2023-07-12T00:09:14.492Z","end_time":"2023-07-12T00:10:14.492Z","floating":false,"unit":"шт","order_id":7,"status":0,"plan_salary":153.0102,"pass_salary":0,"accept_salary":0,"plan_cost_per_hour":15.30102,"pass_cost_per_hour":0,"accept_cost_per_hour":0,"plan_count":1.02,"duration":1}],"plan_salary":1027.6602,"pass_salary":0,"accept_salary":0}]"#;

		let des: Vec<Task> = serde_json::from_str(data).unwrap();
		println!("{:#?}", des);
	}
    #[test]
    fn alarms() {
		// a!(StartOperation, operation 24, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(EndOperation, operation 24, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(EndOperationRest, operation 24, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(EndOperationLast, operation 24, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(NoOperation, operation 24, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(StartTask, task 5, "12:15", "Отъедка", "Текущая операция");
		// a!(EndTask, task 5, "12:15", "Отъедка", "Текущая операция");

		// assert_eq!(get_alarm_datas(&vec![
		// 	g!(operation 24, task 5, "Подводка", 1 min, 6/8/9, 24.0 => Open, None, None - None, 0/0),
		// 	g!(operation 25, task 5, "Доводка", 5 min, 45/7/0, 15.0 => Open, None, None - None, 0/0),
		// 	g!(operation 26, task 5, "Отводка", 5 min, 45/7/0, 10.5 => Open, Some(0.0), None - None, 0/0),
		// 	g!(operation 27, task 5, "Отводка", 5 min, 45/7/0, 12.45 => Open, Some(25.0), None - None, 0/0),
		// ]), vec![
		// 	a!(StartOperation, operation 24, task 5, "12:15", "Отъедка", "Текущая операция"),
		// ]);
		
		
		// INSERT INTO tasks ("date", sector_id, start_time, end_time, "name", description)
		// VALUES(CURRENT_DATE, 1, CURRENT_TIME + (1 * interval '1 minute') + (6 * interval '1 hour'), CURRENT_TIME + (12 * interval '1 minute') + (6 * interval '1 hour'), 'Монтаж лоджии', 'Смонтируйте лоджию!');
		
		// INSERT INTO operations (floating, user_id, task_id, "name", unit_id, "count", single_price, start_time, end_time)
		// VALUES(TRUE, user_id, currval(pg_get_serial_sequence('tasks', 'id')), 'Переодевание', 1, 5.1, 24.5, CURRENT_TIME + (1 * interval '1 minute') + (6 * interval '1 hour'), CURRENT_TIME + (2 * interval '1 minute') + (6 * interval '1 hour'));
	
		// INSERT INTO operations (floating, user_id, task_id, "name", unit_id, "count", single_price, start_time, end_time)
		// VALUES(TRUE, user_id, currval(pg_get_serial_sequence('tasks', 'id')), 'Сборка', 1, 5.1, 24.5, CURRENT_TIME + (2 * interval '1 minute') + (6 * interval '1 hour'), CURRENT_TIME + (3 * interval '1 minute') + (6 * interval '1 hour'));
	
		// INSERT INTO operations (floating, user_id, task_id, "name", unit_id, "count", single_price, start_time, end_time)
		// VALUES(TRUE, user_id, currval(pg_get_serial_sequence('tasks', 'id')), 'Монтаж', 1, 5.1, 24.5, CURRENT_TIME + (3 * interval '1 minute') + (6 * interval '1 hour'), CURRENT_TIME + (4 * interval '1 minute') + (6 * interval '1 hour'));
	
		// INSERT INTO plans (task_id, user_id) VALUES(currval(pg_get_serial_sequence('tasks', 'id')), user_id)
		// $procedure$

    }
}

fn get_operation_duration(oo: &(Operation, OperationResult), current_operation_id: u32) -> Duration {
	if oo.0.id == current_operation_id {
		Duration::minutes(oo.0.duration.unwrap() as i64) - oo.1.duration_num()
	} else {
		Duration::minutes(oo.0.duration.unwrap().into())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[pub_this]
pub struct Oor {
	task_id: u32,
	operation_id: u32,
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,
	name: String,
}

fn get_general_block_operations<'a>(operations: &'a Vec<(Operation, OperationResult)>, current_operation_id: u32, is_unpause: bool) -> Vec<Oor> {
	let mut ops: Vec<Oor> = vec![];
	let current_operation_index = operations.iter().position(|it| it.0.id == current_operation_id).unwrap();

	let mut first = operations.get(current_operation_index).unwrap().1.start_time.unwrap_or(Utc::now());
	log_str(&format!("get_general_block_operations first {first}"));
	for i in 0..operations.len() {
		if i < current_operation_index as usize {
			continue;
		}
		let op = &operations.get(i).unwrap();
		if op.0.floating {
			break;
		}
		let o = op.0.clone();
		let b = op.1.clone();
		// let (o, b) = op.clone();
		// let mut or = b.clone();
		let duration = get_operation_duration(&op, current_operation_id);
		let duration_minutes = duration.num_seconds() / 60;
		let duration_seconds = duration.num_seconds() - duration_minutes * 60;
		debug!("duration of op {} {:0>2} m {:0>2} s", o.id, duration_minutes, duration_seconds);
		// or.end_time = Some(first + duration);
		// or.start_time = Some(first);
		
		if is_unpause && o.id == current_operation_id {
			// b.pauses_duration()
			let end_time = Duration::minutes(o.duration.unwrap().into()) + b.pauses_duration();
			ops.push(Oor {
				task_id: o.task_id,
				operation_id: o.id,
				name: o.name,
				start_time: first,
				end_time: first + end_time,
			});
			first += end_time;
		} else {
			let end_time = Duration::minutes(o.duration.unwrap().into());
			ops.push(Oor {
				task_id: o.task_id,
				operation_id: o.id,
				name: o.name,
				start_time: first,
				end_time: first + end_time,
			});
			first += end_time;
		}
	}
	ops
}

pub fn get_alarm_items(operations: &Vec<(Operation, OperationResult)>, current_operation_id: u32, is_unpause: bool) -> (AlarmItems, Vec<Oor>) {
	let mut alarm_datas: Vec<AlarmItem> = vec![];
	let filtered_operations = get_general_block_operations(&operations, current_operation_id, is_unpause);
	log_str(&serde_json::to_string(&filtered_operations).unwrap());
	for i in 0..filtered_operations.len() {
		let op = &filtered_operations.get(i).unwrap();
		let next = &filtered_operations.get(i + 1);
		alarm_datas.push(AlarmItem {
			operation_id: op.operation_id,
			name: op.name.clone(),
			start_time: op.start_time.into(),
			end_time: op.end_time.into(),
			after_rest: next.is_none(),
		});
	}
	(AlarmItems {
		items: alarm_datas,
		task_id: operations.first().unwrap().0.task_id,
	}, filtered_operations)
}

pub fn relog(str: impl Into<String>) {
	let _ = eval(&format!("console.re.log(`{}`)", str.into()));
}

pub fn play_sound(sound: impl Into<String>) {
	let _ = eval(&format!("(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).playSound(`{}`)", sound.into()));
}

static STORAGE_PLANNED_ALARMS_KEY: &'static str = "alarms";

#[logfn(Info)]
pub fn restore_alarms() -> Option<(u32, DateTime<Utc>)> {
	let ais: Option<AlarmItems> = load_from_app_storage("alarms".into());
	if let Some(ais) = ais {
		relog("[restore_alarms] loading alarms");
		let current_operation = ais.get_current_operation();
		let alarm_datas: Vec<Alarm> = ais.into();
		let alarms_ser = serde_json::to_string(&alarm_datas).unwrap();
		let _ = eval(&format!("(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).registerAlarms(`{}`)", alarms_ser));
		relog("[restore_alarms] registered successfull");

		if let Some(operation) = current_operation {
			Some((operation.0, operation.1))
		} else {
			None
		}
	} else {
		relog("[restore_alarms] no saved alarms");
		None
	}
}

#[logfn(Info)]
pub fn register_alarms(state: &mut State, current_operation_id: u32, first_operation_has_pauses: bool) {
	let operations = state.get_operation_pairs_vec();
	// TODO: Здесь получить AlarmItems! Возможно в get_alarm_datas. Возможно параллельно с этими ниже, чтобы ничего не переделывать.
	// Проверить метод get_current_operation_id, чтобы он возвращал верный id текущей операции по исходя из текущего времени

	let (ais, oors) = get_alarm_items(&operations.clone(), current_operation_id, first_operation_has_pauses);

	save_to_app_storage("alarms".into(), &ais);
	let mut alarm_datas: Vec<Alarm> = ais.into();

	// if first_operation_has_pauses {
	// 	log_str("first_operation_has_pauses true");
	// 	if alarm_datas.get(0).unwrap()._type == ActionType::StartOperation {
	// 		alarm_datas.remove(0);
	// 	}
	// }

	relog(&format!("[register_alarms] current time is {}", Local::now()));
	for ad in alarm_datas.iter() {
		relog(&format!("{}", ad));
	}

	let _ = LocalStorage::set(STORAGE_PLANNED_ALARMS_KEY, &alarm_datas);

	for alarm in alarm_datas.iter() {
		if let Some(aa) = &alarm.alarm_action {
			println!("{}", aa.js_action);
		}
	}

	state.alarms = Some(oors);

	log_str(&format!("[register_alarms_x] {:#?}", &alarm_datas));
	save_to_app_storage("alarms".into(), &alarm_datas);
	run_void_command("registerAlarms", &alarm_datas);
}

#[logfn(Info)]
pub fn register_alarms_from_alarms_vec(alarm_datas: &Vec<Alarm>) {
	relog(&format!("[register_alarms_from_alarms_vec] current time is {}", Local::now()));
	for ad in alarm_datas.iter() {
		relog(&format!("{}", ad));
	}

	let _ = LocalStorage::set(STORAGE_PLANNED_ALARMS_KEY, &alarm_datas);

	for alarm in alarm_datas.iter() {
		if let Some(aa) = &alarm.alarm_action {
			println!("{}", aa.js_action);
		}
	}

	log_str(&format!("[register_alarms_x] {:#?}", &alarm_datas));
	save_to_app_storage("alarms".into(), &alarm_datas);
	run_void_command("registerAlarms", &alarm_datas);
}

pub fn run_void_command<T>(command: &str, json_object_as_string_arg: &T) where T: Serialize, T: Debug  {
	let serialized = serde_json::to_string(json_object_as_string_arg).unwrap();
	// let _ = eval(&format!("console.log(`{}`)", serialized)).unwrap();
	let _ = eval(&format!("window.runVoidCommand('{}', `{}`)", command, serialized)).unwrap();
}

pub fn run_void_command_empty_array_arg(command: &str)  {
	let _ = eval(&format!("window.runVoidCommand('{}', `[]`)", command)).unwrap();
}

#[logfn(Info)]
pub fn cancel_all_alarms(state: &mut State) {
	state.alarms = None;
	relog("console.re.log('cancelling all alarms')");
	run_void_command_empty_array_arg("registerAlarms");
}

#[logfn(Info)]
pub fn warning_message(title: &str, message: &str) {
	let _ = eval(&format!("warningMessage(\"{title}\", \"{message}\")")).unwrap();
}

#[logfn(Info)]
pub fn error_message(title: &str, message: &str) {
	let _ = eval(&format!("errorMessage(\"{title}\", \"{message}\")")).unwrap();
}

#[logfn(Info)]
pub fn success_message(title: &str, message: &str) {
	let _ = eval(&format!("successMessage(\"{title}\", \"{message}\")")).unwrap();
}

#[logfn(Info)]
pub fn load_from_app_storage<T>(key: String) -> Option<T> where T: DeserializeOwned, T: Debug {
	relog("load_from_app_storage#1");
	let eval_result = eval(&format!(
		"(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).loadFromStorage('{}');",
		key,
	));
	relog("load_from_app_storage#2");
	let Ok(string_value) = eval_result else {
		relog(format!("load_from_app_storage#2e"));
		return Option::<T>::None;
	};
	relog(format!("load_from_app_storage#3 {:?}", string_value));
	let Some(string_value) = string_value.as_string() else {
		return Option::<T>::None;
	};
	return serde_json::from_str::<T>(&string_value).ok();
}

#[logfn(Info)]
pub fn save_to_app_storage<T>(key: String, object: &T) -> bool where T: Serialize, T: Debug {
	relog("save_to_app_storage#1");
	let Ok(_) = eval(&format!(
		"(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).saveToStorage('{}', `{}`);",
		key,
		serde_json::to_string(object).unwrap()
	)) else {
		relog("save_to_app_storage#2");
		return false;
	};
	return true;	
}

#[logfn(Info)]
pub fn clear_app_storage(key: String) -> bool {
	relog("clear_app_storage#1");
	let Ok(_) = eval(&format!(
		"(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).clearStorage('{}');",
		key,
	)) else {
		relog("clear_app_storage#2");
		return false;
	};
	return true;	
}

#[logfn(Info)]
pub fn system_navigate(screen: &str) -> bool {	
	let Ok(_) = eval(&format!(
		"(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).systemNavigate('{}');",
		screen,
	)) else {
		relog("system_navigate#2");
		return false;
	};
	return true;	
}

#[logfn(Info)]
pub fn register_fcm_token() -> bool {	
	let Ok(_) = eval("(Bridge?.interfaces?.Android || window.JSInterfaceDesktop).registerFcmToken();") else {
		relog("register_fcm#2");
		return false;
	};
	return true;	
}

#[logfn(Info)]
pub fn get_first_passed_operation_id(operation_results: &BTreeMap<u32, OperationResult>, operation_results_order: &Vec<u32>) -> Option<u32> {
	let ordered_operations = operation_results_order.iter().map(|op| operation_results.get(op).unwrap()).collect::<Vec<_>>();
	let first_passed_operation = ordered_operations.iter().find(|op| op.status == OperationResultStatus::Pass);
	if let Some(first_passed_operation) = first_passed_operation {
		return Some(first_passed_operation.operation_id);
	} else {
		return None;
	}
}

#[logfn(Info)]
pub fn get_second_passed_operation_id(operation_results: &BTreeMap<u32, OperationResult>, operation_results_order: &Vec<u32>) -> Option<u32> {
	let ordered_operations = operation_results_order.iter().map(|op| operation_results.get(op).unwrap()).collect::<Vec<_>>();
	let all_passed_operation = ordered_operations.iter().filter(|op| op.status == OperationResultStatus::Pass).collect::<Vec<_>>();
	if let Some(second_passed_operation) = all_passed_operation.get(1) {
		return Some(second_passed_operation.operation_id);
	} else {
		return None;
	}
}

pub async fn post_unauth<'a, T, B>(path: &'a str, body: &B) -> Result<T, ApiError> where T: DeserializeOwned, T: 'static, B: Serialize {
	let response = reqwest::Client::new().post(format!("{BACKEND_URL}{path}"))
		.header(reqwest::header::CONTENT_TYPE, "application/json")
		.body(serde_json::to_string(&body).unwrap())
		.send()
		.await;

	match response {
		Ok(response) => {
			if response.status() == 200 {
				Ok(response.json::<T>().await.unwrap())
			} else {
				Err(response.json::<ApiError>().await.unwrap_or(ApiError { error: "Ошибка авторизации (вероятно неверный логин или пароль)".to_string(), global: true, stack: Some("".to_string()), status: 500 }))
			}
		},
		Err(error) => Err(serde_json::from_str::<ApiError>(&error.to_string()).unwrap_or(ApiError { error: "Ошибка авторизации (вероятно неверный логин или пароль)".to_string(), global: true, stack: Some("".to_string()), status: 500 })),
	}
}

pub async fn post_body<'a, T, B>(path: &'a str, session: &'a str, body: &B) -> Result<T, ApiError> where T: DeserializeOwned, T: 'static, B: Serialize {
	log_str("session");
	log_str(session);

	let response = reqwest::Client::new().post(format!("{BACKEND_URL}{path}"))
		.header(reqwest::header::CONTENT_TYPE, "application/json")
		.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &session))
		.body(serde_json::to_string(&body).unwrap())
		.send()
		.await;

	match response {
		Ok(response) => {
			if response.status() == 200 {
				Ok(response.json::<T>().await.unwrap())
			} else {
				Err(response.json::<ApiError>().await.unwrap())
			}
		},
		Err(error) => Err(serde_json::from_str::<ApiError>(&error.to_string()).unwrap_or(ApiError { error: "Ошибка POST+BODY запроса".to_string(), global: true, stack: Some("".to_string()), status: 500 })),
	}
}

pub async fn post<'a, T>(path: &'a str, session: &'a str) -> Result<T, ApiError> where T: DeserializeOwned, T: 'static {
	relog(path);
	relog(session);
	let response = reqwest::Client::new().post(format!("{BACKEND_URL}{path}"))
		.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &session))
		.send()
		.await;

	match response {
		Ok(response) => {
			if response.status() == 200 {
				Ok(response.json::<T>().await.unwrap())
			} else {
				Err(response.json::<ApiError>().await.unwrap())
			}
		},
		Err(error) => Err(serde_json::from_str::<ApiError>(&error.to_string()).unwrap_or(ApiError { error: "Ошибка POST запроса".to_string(), global: true, stack: Some("".to_string()), status: 500 })),
	}
}

pub async fn get<'a, T>(path: &'a str, session: &'a str) -> Result<T, ApiError> where T: DeserializeOwned, T: 'static {
	let response = reqwest::Client::new().get(format!("{BACKEND_URL}{path}"))
		.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &session))
		.send()
		.await;

	match response {
		Ok(response) => {
			if response.status() == 200 {
				Ok(response.json::<T>().await.unwrap())
			} else {
				Err(response.json::<ApiError>().await.unwrap())
			}
		},
		Err(error) => Err(serde_json::from_str::<ApiError>(&error.to_string()).unwrap_or(ApiError { error: "Ошибка GET запроса".to_string(), global: true, stack: Some("".to_string()), status: 500 })),
	}
}

pub fn scroll_to_operation(operation_id: u32) {
	eval(&format!("scrollToOperation({})", operation_id)).unwrap();
}

// pub fn use_http<'a, T>(cx: &'a ScopeState, path: &'a str, session: &'a str) -> &'a UseFuture<Result<T, Error>> where T: DeserializeOwned, T: 'static {
// 	use_future(cx, (&path.to_string(), &session.to_string()), |(path, session)| async move {
// 		reqwest::Client::new().get(format!("{BACKEND_URL}{path}"))
// 			.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &session))
// 			.send()
// 			.await
// 			.unwrap() // TODO!
// 			.json::<T>()
// 			.await
// 	})
// }

// pub fn use_admin_http<'a, T>(cx: &'a ScopeState, path: &str) -> &'a UseFuture<Result<T, Error>> where T: DeserializeOwned, 
// 	T: 'static {
// 	use_future(cx, &path.to_string(), |path| async move {
// 		reqwest::Client::new().get(format!("{BACKEND_URL}{path}"))
// 			.header(reqwest::header::AUTHORIZATION, format!("Bearer {PLANNERKEY}"))
// 			.send()
// 			.await
// 			.unwrap() // TODO!
// 			.json::<T>()
// 			.await
// 	})
// }

#[macro_export]
macro_rules! log {
	($($el: tt)*) => {
		fn argento(arr: &mut js_sys::Array, s: impl Into<String>) {
			arr.set(0, wasm_bindgen::JsValue::from_str(&s.into()));
		}
		let mut arr = js_sys::Array::new();
		$(
			argento(&mut arr, &$el);
		)*
		web_sys::console::log(&arr);
	};
}

pub fn log_str(str: &str) {
	let arr = js_sys::Array::new();
	arr.set(0, JsValue::from_str(str));
	console::log(&arr);
}

pub fn string_time_to_date(time: &str) -> DateTime<Utc> {
	let cpt = time.split(":").collect::<Vec<_>>();
	let current_date = chrono::Utc::now().date_naive();
	let date_time = chrono::Utc.with_ymd_and_hms(
		current_date.year(), 
		current_date.month(),
		current_date.day(), 
		cpt[0].parse::<u32>().unwrap(),
		cpt[1].parse::<u32>().unwrap(), 
		0
	).unwrap();
	date_time
}