use std::{collections::{HashMap, BTreeMap}, borrow::BorrowMut};
use crate::{*, models::OperationResultStatus, utils::{register_alarms, cancel_all_alarms, play_sound, Oor}};
use gloo::storage::{LocalStorage, Storage};
use log_derive::logfn;
use tap::Tap;
use indextreemap::IndexTreeMap;

#[derive(Default, Clone, PartialEq, Store, Serialize, Deserialize)]
#[store(storage = "local")]
#[pub_this]
pub struct State {
	user_session_id: Option<String>,
	user: Option<User>,
	// FIX MeC24d
	taken_tasks_count: i8,
	new_tasks_count: i8,
	finished_tasks_count: i8,
	new_tasks: Option<Vec<Task>>,
	taken_tasks: Option<Vec<Task>>,
	finished_tasks: Option<Vec<Task>>,
	active_task: Option<ActiveTask>,
	alarms: Option<Vec<Oor>>,
}

pub fn get_current_operation(alarms: &Vec<Oor>) -> Option<(u32, DateTime<Utc>)> {
	let now = Utc::now().time();
	if let Some(alarm_item) = alarms.iter().find(|it| {
		let start_time = it.start_time.clone().time();
		let end_time = it.end_time.clone().time();
		(start_time..end_time).contains(&now)
	}) {
		Some((alarm_item.operation_id, alarm_item.start_time.clone()))
	} else {
		None
	}
}

pub fn get_non_passed_operations(alarms: &Vec<Oor>) -> Vec<(u32, DateTime<Utc>)> {
	let now = Utc::now().time();
	alarms.iter().filter(|it| {
		let start_time = it.start_time.clone().time();
		let end_time = it.end_time.clone().time();
		start_time < now
	}).map(|it| (it.operation_id, it.start_time.clone())).collect::<Vec<_>>()
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[pub_this]
pub struct ActiveTask {
	task: Task,
	// current_operation: Option<CurrentOperation>,
	current_operation_id: Option<u32>,
	operation_results: BTreeMap<u32, OperationResult>,
	operation_results_order: Vec<u32>,
}

impl ActiveTask {
	pub fn current_operation_result(&self) -> Option<OperationResult> {
		let Some(current_operation_id) = self.current_operation_id else { return None };
		Some(self.operation_results[&current_operation_id].clone())
	}

	pub fn current_operation(&self) -> Option<Operation> {
		let Some(operation_result) = self.current_operation_result() else { return  None };
		Some(operation_result.get_operation_from_task(self))
	}
}

static STORAGE_STATE_KEY: &'static str = "state";

pub trait AppStateActions {
	fn get_operation_pairs_vec(&mut self) -> Vec<(Operation, OperationResult)>;
	fn launch_task(&mut self, task: &Task, auto_start_first_operation: bool);
	fn launch_task_from_taken(&mut self, task: &Task);
	fn launch_task_from_new(&mut self, task: &Task);
	fn start_operation(&mut self, operation_id: u32);
	fn start_operation_with_time(&mut self, operation_id: u32, time: DateTime<Utc>);
	fn finish_operation(&mut self, operation_id: u32, count: f32, comment: Option<String>, end_time: DateTime<Utc>) -> &OperationResult;
	fn operation_upload(&mut self, operation_id: u32);
	fn operation_pass_upload(&mut self, operation_id: u32, go_next: bool);
	fn operation_success_upload(&mut self, operation_id: u32);
	fn operation_failure_upload(&mut self, operation_id: u32);
	fn next_operation(&mut self, operation_id: u32);
	// fn get_active_operation_result_mut(&mut self) -> Option<&mut OperationResult>;
	fn get_active_operation_result(&mut self) -> Option<OperationResult>;
	// fn change_operation_result_mut<'a, F: FnOnce(&mut OperationResult)>(&self, f: F) -> bool;
	fn pause_operation(&mut self, operation_id: u32); 
	fn pause_operation_with_pause(&mut self, operation_id: u32, comment: &str);
	fn resume_operation(&mut self, operation_id: u32);
	fn resume_operation_with_comment(&mut self, operation_id: u32, comment: &str);
	fn start_initial_general_operation(&mut self, operation_id: u32);
	fn no_operation(&mut self);

	fn not_done_operations_count(&self) -> u32;
	fn not_uploaded_operations_count(&self) -> u32;
	fn remaining_operations(&self) -> u32;
}

fn get_operation_next_id(active_task: &mut ActiveTask, cur_operation_index: usize) -> Option<u32> {
	let or = active_task.operation_results.iter().enumerate().find(|it| it.0 == cur_operation_index);
	match or {
		Some(or) => {
			Some(*or.1.0)
		},
		_ => None,
	}
}

#[logfn(Info)]
pub fn get_next_operation_id(active_task: &ActiveTask, operation_id: u32) -> Option<u32> {
	let mut cur_operation_index = active_task.operation_results_order.iter().position(|it| *it == operation_id).unwrap();
	cur_operation_index += 1;
	if cur_operation_index < active_task.operation_results_order.len() {
		let next_operation_id = active_task.operation_results_order[cur_operation_index];
		Some(next_operation_id)
	} else {
		None
	}
}

#[logfn(Info)]
pub fn next_operation(active_task: &mut ActiveTask, operation_id: u32) -> Option<u32> {
	let mut cur_operation_index = active_task.operation_results_order.iter().position(|it| *it == operation_id).unwrap();
	cur_operation_index += 1;
	if cur_operation_index < active_task.operation_results_order.len() {
		let next_operation_id = active_task.operation_results_order[cur_operation_index];
		let or = active_task.operation_results.get_mut(&next_operation_id).unwrap();
		or.start_time = Some(Utc::now());
		
		or.pass();
		active_task.current_operation_id = Some(or.operation_id);
		Some(or.operation_id)
	} else {
		None
	}
	// let mut cur_operation_index = active_task.operation_results.iter().position(|it| *it.0 == operation_id).unwrap();
	// cur_operation_index += 1;
	// if let Some(next_op_id) = get_operation_next_id(active_task, cur_operation_index) {
	// 	let or = active_task.operation_results.get_mut(&next_op_id).unwrap();
	// 	or.start_time = Some(Utc::now());
		
	// 	or.pass();
	// 	active_task.current_operation_id = Some(or.operation_id);
	// 	Some(or.operation_id)
	// } else {
	// 	None
	// }
}

fn lift_up_operation_result(operation_results: &BTreeMap<u32, OperationResult>, operation_results_order: &Vec<u32>, operation_id: u32) -> Vec<u32> {
	let removed = operation_results_order.iter().position(|it| it == &operation_id).unwrap();

	// let last_pass_position = active_task.operation_results.iter().rev().position(|it| it.1.status == OperationResultStatus::Pass).unwrap_or(0);

	let passed_ors = operation_results_order.iter().filter(|it| {
		let or = operation_results.get(&**it).unwrap();
		or.status != OperationResultStatus::Open && or.operation_id != operation_id
	}).collect::<Vec<_>>();
	// debug!("passed_ors {}", serde_json::to_string(&passed_ors.iter().map(|it| *it.0).collect::<Vec<u32>>()).unwrap());

	let non_passed_ors = operation_results_order.iter().filter(|it| {
		let or = operation_results.get(&**it).unwrap();
		or.status == OperationResultStatus::Open && or.operation_id != operation_id
	}).collect::<Vec<_>>();
	// debug!("non_passed_ors {}", serde_json::to_string(&non_passed_ors.iter().map(|it| *it.0).collect::<Vec<u32>>()).unwrap());

	let mut new_ors: Vec<u32> = Vec::new();
	for or in passed_ors.into_iter() {
		new_ors.push(*or);
	}
	new_ors.push(operation_id);
	for or in non_passed_ors.into_iter() {
		new_ors.push(*or);
	}

	debug!("{}", serde_json::to_string(&new_ors).unwrap());

	new_ors
}

impl AppStateActions for State  {
	fn remaining_operations(&self) -> u32 {
		let Some(active_task) = self.active_task.as_ref() else {
			return 99999;
		};
		active_task.operation_results_order.iter().filter(|it| {
			let operation_result = active_task.operation_results.get(&it).unwrap().clone();
			if let Some(current_operation_id) = active_task.current_operation_id && current_operation_id == **it {
				true
			} else {
				match operation_result.status {
					OperationResultStatus::Open => true,
					OperationResultStatus::Pass => false,
					OperationResultStatus::Done(done_status) => false,
				}
			}
		}).count() as u32
	}

	fn not_done_operations_count(&self) -> u32 {
		let Some(active_task) = self.active_task.as_ref() else {
			return 99999;
		};
		active_task.operation_results_order.iter().filter(|it| {
			let operation_result = active_task.operation_results.get(&it).unwrap().clone();
			match operation_result.status {
				OperationResultStatus::Open => true,
				OperationResultStatus::Pass => true,
				OperationResultStatus::Done(done_status) => false,
			}
		}).count() as u32
	}

	fn not_uploaded_operations_count(&self) -> u32 {
		let Some(active_task) = self.active_task.as_ref() else {
			return 99999;
		};
		active_task.operation_results_order.iter().filter(|it| {
			let operation_result = active_task.operation_results.get(&it).unwrap().clone();
			match operation_result.status {
				OperationResultStatus::Open => true,
				OperationResultStatus::Pass => true,
				OperationResultStatus::Done(done_status) => {
					if let OperationDoneStatus::Successfull = done_status {
						false
					} else {
						true
					}
				},
			}
		}).count() as u32
	}

	#[logfn(Info)]
	fn pause_operation(&mut self, operation_id: u32) {
		let active_task = self.active_task.as_mut().unwrap();
		let operation = active_task.task.operations.iter().find(|it| it.id == operation_id).unwrap();
		active_task
			.operation_results.get_mut(&operation_id).unwrap()
			.pauses.push(Pause {
				pause_time: chrono::Utc::now(),
				resume_time: None,
				comment: None,
			});
		if !operation.floating {
			// Остановить все Алармы!
			cancel_all_alarms(self);
		}
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn pause_operation_with_pause(&mut self, operation_id: u32, comment: &str) {
		let active_task = self.active_task.as_mut().unwrap();
		let operation = active_task.task.operations.iter().find(|it| it.id == operation_id).unwrap();
		relog(format!("active_task.operation_results {:#?}", active_task.operation_results));
		active_task
			.operation_results.get_mut(&operation_id).unwrap()
			.pauses.push(Pause {
				pause_time: chrono::Utc::now(),
				resume_time: None,
				comment: Some(comment.to_string()),
			});
		if !operation.floating {
			// Остановить все Алармы!
			cancel_all_alarms(self);
		}
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn resume_operation(&mut self, operation_id: u32) {
		let active_task = self.active_task.as_mut().unwrap();
		active_task
			.operation_results.get_mut(&operation_id).unwrap()
			.pauses.last_mut().unwrap()
			.resume_time = Some(chrono::Utc::now());
		let operation = active_task.task.operations.iter().find(|it| it.id == operation_id).unwrap();
		if !operation.floating {
			if let Some(cur_op_res) = active_task.current_operation_result() {
				register_alarms(self, operation_id, !cur_op_res.pauses.is_empty());
			} else {
				register_alarms(self, operation_id, false);
			}
		}		
	}

	#[logfn(Info)]
	fn resume_operation_with_comment(&mut self, operation_id: u32, comment: &str) {
		let active_task = self.active_task.as_mut().unwrap();
		active_task
			.operation_results.get_mut(&operation_id).unwrap()
			.pauses.last_mut().unwrap()
			.tap_mut(|it| {
				it.resume_time = Some(chrono::Utc::now());
				it.comment = Some(comment.into());
			});
			
		let operation = active_task.task.operations.iter().find(|it| it.id == operation_id).unwrap();
		if !operation.floating {
			if let Some(cur_op_res) = active_task.current_operation_result() {
				register_alarms(self, operation_id, !cur_op_res.pauses.is_empty());
			} else {
				register_alarms(self, operation_id, false);
			}
		}		
	}

	// fn get_active_operation_result_mut(&self) -> Option<&OperationResult> { }

	// fn change_operation_result_mut<'a, F: FnOnce(&mut OperationResult)>(&self, f: F) -> bool {
	// 	let active_task = self.active_task.as_mut().unwrap();
	// 	if let Some(current_operation_id) = active_task.current_operation_id {
	// 		if let Some(operation_result) = active_task.operation_results.get_mut(&current_operation_id) {
	// 			f(operation_result);
	// 			true
	// 		} else {
	// 			false
	// 		}
	// 	} else {
	// 		false
	// 	}
	// }

	#[logfn(Info)]
	fn get_active_operation_result(&mut self) -> Option<OperationResult> {
		// TODO!
		let state = self;
		let active_task = state.active_task.as_ref().unwrap();
		if let Some(current_operation_id) = active_task.current_operation_id {
			if let Some(operation_result) = active_task.operation_results.get(&current_operation_id) {
				Some(operation_result.clone())
			} else {
				None
			}
		} else {
			None
		}
	}

	#[logfn(Info)]
	fn get_operation_pairs_vec(&mut self) -> Vec<(Operation, OperationResult)> {
		let active_task = self.active_task.as_ref().expect("Нет активной задачи");
		active_task.operation_results_order.iter().map(|it| {
			let operation_result = active_task.operation_results.get(it).unwrap();
			(operation_result.get_operation_from_task(&active_task).clone(), operation_result.clone())
		}).collect::<Vec<_>>()
	}

	#[logfn(Info)]
	fn launch_task_from_taken(&mut self, task: &Task) {
		self.taken_tasks_count -= 1;
		self.launch_task(task, false);
	}

	#[logfn(Info)]
	fn launch_task_from_new(&mut self, task: &Task) {
		println!("launch_task_from_new asd");
		self.new_tasks_count -= 1;
		self.launch_task(task, false);
	}

	
	#[logfn(Info)]
	fn launch_task(&mut self, task: &Task, auto_start_first_operation: bool) {
		debug!("launch_task1");
		/* Временно */
		let ors = BTreeMap::from_iter(
			task.operations.iter().map(|o| (o.id, o.into()))
		);
		debug!("launch_task2");
		// let current_operation_id: Option<u32> = ;
		debug!("launch_task3");
		self.active_task = Some(ActiveTask {
			operation_results: ors,
			operation_results_order: task.operations.iter().map(|it| it.id).collect::<Vec<_>>(),
			current_operation_id: None,
			task: task.clone(),
		});

		if auto_start_first_operation {
			self.start_initial_general_operation(task.operations.first().unwrap().id);
		}

		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	fn start_initial_general_operation(&mut self, operation_id: u32) {
		let mut active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.current_operation_id = Some(operation_id);

		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.start_time = Some(chrono::Utc::now());
			o.pass();
		});
		debug!("old order {:#?}", active_task.operation_results_order);
		active_task.operation_results_order = lift_up_operation_result(&active_task.operation_results, &active_task.operation_results_order, operation_id);
		debug!("new order {:#?}", active_task.operation_results_order);
		// self.start_operation(operation_id);

		if let Some(cur_op_res) = active_task.current_operation_result() {
			register_alarms(self, operation_id, !cur_op_res.pauses.is_empty());
		} else {
			register_alarms(self, operation_id, false);
		}
		
		play_sound("short_sound.mp3");
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn start_operation(&mut self, operation_id: u32) { // По сути делается Pass
		let mut active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.current_operation_id = Some(operation_id);

		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			// o.start_time = Some(chrono::Utc::now());
			o.pass();
		});
		debug!("old order {:#?}", active_task.operation_results_order);
		active_task.operation_results_order = lift_up_operation_result(&active_task.operation_results, &active_task.operation_results_order, operation_id);
		debug!("new order {:#?}", active_task.operation_results_order);
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn start_operation_with_time(&mut self, operation_id: u32, time: DateTime<Utc>) {
		let mut active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.current_operation_id = Some(operation_id);

		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.start_time = Some(time);
			o.pass();
		});
		debug!("old order {:#?}", active_task.operation_results_order);
		active_task.operation_results_order = lift_up_operation_result(&active_task.operation_results, &active_task.operation_results_order, operation_id);
		debug!("new order {:#?}", active_task.operation_results_order);
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn finish_operation<'a>(&'a mut self, operation_id: u32, count: f32, comment: Option<String>, end_time: DateTime<Utc>) -> &'a OperationResult {
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");

		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.count = Some(count);
			o.comment = comment;
			o.end_time = Some(end_time);
			o.upload();
		})
	}

	#[logfn(Info)]
	fn next_operation(&mut self, operation_id: u32) {
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");
		let operations = &active_task.task.operations;
		let op_pos = operations.iter().position(|o| o.id == operation_id).unwrap();
		if op_pos + 1 < operations.len() {
			let op = &operations[op_pos + 1];
			active_task.current_operation_id = Some(op.id);
			// CurrentOperation {
			// 	operation_id: op.id,
			// 	start_time: chrono::Utc::now(),
			// 	pauses: vec![],
			// 	delta: 0,
			// 	total_delta: 0,
			// }
		} else {
			active_task.current_operation_id = None;
		}
	}

	#[logfn(Info)]
	fn operation_upload(&mut self, operation_id: u32) {
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.status = OperationResultStatus::Done(OperationDoneStatus::Uploading);
		});
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}

	#[logfn(Info)]
	fn no_operation(&mut self) {
		if let Some(at) = self.active_task.as_mut() {
			at.current_operation_id = None;
		}
		// let _ = LocalStorage::set(STORAGE_STATE_KEY, self);
	}
	
	#[logfn(Info)]
	fn operation_pass_upload(&mut self, operation_id: u32, go_next: bool) {
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");
		if go_next && let Some(next_operation_id) = next_operation(active_task, operation_id) {
			relog("opu!!!ZZZ");
			register_alarms(self, next_operation_id, false);
		} else {
			active_task.current_operation_id = None;
		}
	}

	#[logfn(Info)]
	fn operation_success_upload(&mut self, operation_id: u32) {
		log_str("operation_success_upload");
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.status = OperationResultStatus::Done(OperationDoneStatus::Successfull);
		});
	}

	#[logfn(Info)]
	fn operation_failure_upload(&mut self, operation_id: u32) {
		log_str("operation_failure_upload");
		let active_task = self.active_task.as_mut().expect("Нет активной задачи");
		active_task.operation_results.get_mut(&operation_id).unwrap().tap_mut(|o| {
			o.status = OperationResultStatus::Done(OperationDoneStatus::Failure);
		});
	}
}
